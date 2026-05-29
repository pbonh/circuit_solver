---
title: 'Graphs in VLSI — Chapter 2: Graph Fundamentals'
type: source
id: summaries/graphs-in-vlsi-05-2-graph-fundamentals
kind: publication
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
---

## Key Points

- A graph is formally an ordered triple G = (V, E, ψ) where ψ : E → V × V is the incidence function. Order is |V|, size is |E|. The handshaking lemma (Euler, 1736) follows from sum of degrees = 2|E|.
- Topological categories include hypergraph (edges may connect any subset of nodes), pseudograph (allows loops and parallel edges), multigraph (parallel edges, no loops), graph with loops (loops, no parallel edges), and simple graph (neither). Weighted graphs and directed graphs are orthogonal subcategories.
- Hypergraphs are useful in VLSI physical design where multiple gates connect through a single wire (a hyperedge).
- A complete graph Kn has n(n-1)/2 edges; weight functions w : E → R define edge-weighted graphs; strength is the weighted degree generalization.
- Directed graphs (digraphs) have ordered-pair edges; indegree, outdegree, source, and sink are basic terminology. A DAG (directed acyclic graph) admits a topological ordering f : V → {1,...,|V|} such that (u,v) ∈ E ⇒ f(u) < f(v).
- Isomorphism is a bijection preserving edge structure; subgraphs and induced subgraphs are basic relationships.
- Walks, trails (no repeated edges), and paths (no repeated nodes) are introduced. A circuit is a closed trail; a cycle has no repeated nodes; a Hamiltonian cycle visits every node.
- Trees are connected acyclic simple graphs (|E| = |V|−1). Rooted trees, m-ary, balanced, complete, and full trees are defined. Bipartite graphs admit a 2-partition with all edges between partitions and are characterized by absence of odd cycles.
- Pathfinding algorithms covered: DFS (Trémaux 19th c.; Tarjan 1972) using a stack, complexity O(|V|+|E|), not guaranteed shortest; BFS (Moore, 1959) using a queue, gives shortest path in unweighted graphs in O(|V|+|E|); Dijkstra (1956) for non-negative weights, O((|V|+|E|) log |V|) with heaps; Bellman-Ford (Shimbel 1954, Ford 1956, Bellman 1958) for negative weights, O(|V||E|), detects negative cycles in |V|-th iteration; A* with admissible heuristics for guided search.
- Spanning-tree algorithms covered: Borůvka (1926), Prim/Jarník (1929), Kruskal (1956); all greedy and produce optimal MST. Advanced algorithms include Fredman-Tarjan (1987) at O(|E| + |V| log* |V|), Chazelle's O(|E| α(|E|,|V|)), and expected-linear-time Karger et al. (1995).
- Steiner Minimum Tree (SMT) is NP-hard but admits a 2-approximation via metric closure MST; better approximations: (1+ln 3/2) ≈ 1.55 (Robins-Zelikovsky), ln 4 ≈ 1.39 (Byrka et al.). Rectilinear SMT and the Hanan grid (1966) are central to VLSI routing. Variants include length-restricted SMT and obstacle-avoiding SMT.
- Graph coloring assigns colors to nodes so adjacent nodes differ; chromatic number χ(G). Variants: equitable coloring, edge coloring (Vizing's theorem: χ′ is Δ or Δ+1), fractional coloring. The Four Color Theorem (Appel & Haken, 1977) drove much of the field.
- Topological sorting via Kahn's algorithm (zero-indegree queue/stack) or DFS-based reverse-removal. Both are O(|V|+|E|); Kahn naturally detects cycles, DFS requires explicit marking.

## Relevant Concepts

- [[concepts/graph-theory]] — the chapter is a self-contained primer on graph theory used by the book.
- [[concepts/hypergraph]] — superclass of graphs needed to model multi-pin VLSI nets.
- [[concepts/directed-acyclic-graph]] — appears throughout VLSI (combinational logic, task scheduling).
- [[concepts/spanning-tree]] — building block for MST and Steiner approximations.
- [[concepts/minimum-spanning-tree]] — classical greedy-solvable optimization problem.
- [[concepts/steiner-minimal-tree]] — NP-hard generalization with central VLSI relevance.
- [[concepts/hanan-grid]] — Hanan's 1966 result restricting RSMT search space.
- [[concepts/depth-first-search]] — stack-based traversal, Tarjan's algorithmic form.
- [[concepts/breadth-first-search]] — queue-based traversal giving shortest paths.
- [[concepts/dijkstras-algorithm]] — non-negative-weight shortest path.
- [[concepts/bellman-ford-algorithm]] — negative-weight shortest path and negative-cycle detection.
- [[concepts/a-star-algorithm]] — heuristic-guided shortest path, used in VLSI grid routing.
- [[concepts/graph-coloring]] — chromatic number, four color theorem.
- [[concepts/topological-sort]] — Kahn's and DFS algorithms for DAG ordering.
- [[concepts/bipartite-graph]] — characterized by absence of odd cycles.
- [[concepts/tree-graph]] — connected acyclic simple graph.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 2 — Graph fundamentals
- File path: `raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
