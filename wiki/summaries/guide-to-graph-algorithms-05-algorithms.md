---
title: Guide to Graph Algorithms — Algorithms (Chapter 2)
type: source
id: source-guide-to-graph-algorithms-05-algorithms
kind: derived-summary
tags:
- graph
- algorithm
- foundational
- well-established
- np-hard
- matching
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
---

## Key Points

- The chapter opens with a Dijkstra-style warning that debugging cannot prove correctness, and Minsky's proof (via a self-referential procedure) that no general termination tester can exist.
- Finding/counting small induced subgraphs: triangles can be detected via fast matrix multiplication in O(n^α) with α < 2.376; Alon-Yuster-Zwick (1997) find a triangle in O(m^(2α/(α+1))) = O(m^1.41). The chapter develops an O(n^α + m^(3/2)) diamond-detection algorithm using a low-degree / high-degree split.
- Bottleneck domination: for a weighted graph (G, w), the optimal dominating-set bottleneck equals ρ = max_x min_{y∈N[x]} w(y), giving a linear-time algorithm; analogous results hold for total dominating sets.
- The Bron-Kerbosch algorithm (Algorithm 3) lists all maximal cliques by maintaining sets R (current clique), P (candidates), X (excluded). Runtime is O(n^2 · 3^(n/3)); the Moon-Moser bound g(n) on maximal cliques uses the same 3^(n/3) ceiling.
- Total ordering: simple total ordering reduces to topological sort of a DAG, solvable in O(n + m) via Kahn's algorithm (Algorithm 4). The general total ordering problem (with betweenness constraints) is NP-complete via reduction from 2-coloring of rank-3 hypergraphs (Opatnrý's reduction).
- NP-completeness is introduced: NP is the class of problems whose oracle-supplied answers can be verified in polynomial time; a problem is NP-complete if all NP problems reduce to it in polynomial time. Equivalence cover number of splitgraphs (q(G) related to chromatic index by Vizing/Holyer) is shown NP-complete.
- The Lovász Local Lemma: if every bad event has probability ≤ p and is independent of all but d others, then P(no bad event) > 0 whenever e·p·(d+1) ≤ 1. Application: 2-colorability of k-uniform hypergraphs with limited intersections. The Moser-Tardos algorithm gives a constructive (resampling) proof via witness trees and a Galton-Watson branching process.
- Szemerédi's Regularity Lemma: for every ε > 0 and t ∈ N, every sufficiently large graph has an ε-regular partition with k+1 classes, t ≤ k ≤ T. Alon et al. (1994) showed deciding ε-regularity is co-NP-complete but produced an O(M(n)) constructive algorithm via neighborhood-deviation σ(p,q).
- Edge-thickness φ(G) and stickiness s(G): φ(G) = 2/(n + s(G)) for graphs without isolated vertices; stickiness is computable via max-flow in O(nm).
- Clique separators: O(n^4) algorithm to list all minimal clique separators, based on feasible partitions {X, S, C} and recursion. The graph can have an exponential number of minimal separators but only at most n minimal clique separators (σ(G) < n).
- Vertex ranking χ_r and permutation graphs: χ_r is computed in O(n^6) for permutation graphs via scanlines and dynamic programming on pieces.
- Cographs (P4-free graphs) and switching cographs are characterized; cographs are recognized via cotree decomposition; switch-equivalent classes give two-graphs.
- Parameterized algorithms: fixed-parameter tractability (FPT) is defined as runtime f(k)·|I|^c. Bounded search technique gives 2^k for vertex cover, 4^k for edge dominating set, and (1.5k)^k for feedback vertex set. Current bests: 1.2738^k for vertex cover, 2.2351^k for edge dominating, 2.7^k randomized for feedback vertex set.
- Matchings ν(G) = α(L(G)); Edmonds' blossom algorithm runs in O(n^2·m), improvable to O(√n · m) by Micali-Vazirani. Minty's algorithm computes α in claw-free graphs in O(n^5) (Faenza et al. improved to O(n^3)).
- Dominoes (every vertex in at most two maximal cliques) are characterized as {W4, claw, gem}-free graphs and recognized in linear time.
- Triangle partition of planar graphs (decomposing E into triangles) is solvable in linear time using bipartiteness of the dual, separating triangles, and Baker's layer decomposition.
- Games on graphs: Snake (player 1 wins iff no perfect matching), Grundy values on DAG positions, De Bruijn's divisor game, NIM via nim-sum (XOR), poset games and Hackendot, coin-turning games, NIM-multiplication forming a field of characteristic 2, Berge's P3-games, and Chomp with the flipping lemma. Chomp on bipartite graphs has Grundy value n_2 + 2·m_2 (mod 2).

## Relevant Concepts

- [[concepts/bron-kerbosch-algorithm]] — lists all maximal cliques in O(n^2 · 3^(n/3))
- [[concepts/maximal-clique]] — clique not contained in a larger one
- [[concepts/topological-sort]] — linear order of DAG respecting arcs
- [[concepts/kahns-algorithm]] — O(n+m) topological sort
- [[concepts/dag]] — directed acyclic graph
- [[concepts/np-completeness]] — formal hardness class
- [[concepts/dominating-set]] — vertices that cover all neighbors
- [[concepts/bottleneck-domination]] — min-max weight domination problem
- [[concepts/lovasz-local-lemma]] — probabilistic existence tool
- [[concepts/moser-tardos-algorithm]] — constructive LLL via resampling
- [[concepts/szemeredi-regularity-lemma]] — partition into ε-regular pairs
- [[concepts/hypergraph]] — generalization of graph with hyperedges
- [[concepts/splitgraph]] — graph partitioning into clique + independent set
- [[concepts/equivalence-cover]] — cover edges with P3-free subgraphs
- [[concepts/chromatic-index]] — minimum edge coloring; χ'(G) ∈ {Δ, Δ+1} by Vizing
- [[concepts/clique-separator]] — separator that induces a clique
- [[concepts/feasible-partition]] — {X, S, C} structure for separator search
- [[concepts/vertex-ranking]] — coloring where same-color vertices need a higher-color separator
- [[concepts/permutation-graph]] — intersection graph of line segments between two parallel lines
- [[concepts/cograph]] — P4-free graph, built by joins and unions
- [[concepts/cotree]] — tree representation of a cograph
- [[concepts/twin]] — pair with identical (closed/open) neighborhood
- [[concepts/fixed-parameter-tractability]] — FPT algorithms parametrized by k
- [[concepts/vertex-cover]] — vertex set covering every edge
- [[concepts/edge-dominating-set]] — edge set whose endpoints dominate
- [[concepts/feedback-vertex-set]] — vertex set hitting every cycle
- [[concepts/matching]] — edge set with no shared endpoint
- [[concepts/edmonds-blossom-algorithm]] — maximum matching in general graphs
- [[concepts/mintys-algorithm]] — max independent set in claw-free graphs
- [[concepts/claw-free-graph]] — no induced K1,3
- [[concepts/domino-graph]] — every vertex in ≤2 maximal cliques
- [[concepts/triangle-partition]] — partitioning edges into triangles
- [[concepts/pq-tree]] — data structure for consecutive-ones property and planarity / interval recognition
- [[concepts/grundy-value]] — game-theoretic value on a position DAG
- [[concepts/nim-sum]] — XOR-based sum of pile sizes for NIM games

## Source Metadata

- Source type: book chapter
- Book title: A Guide to Graph Algorithms
- Chapter 2: Algorithms
- File path: raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
- Authors: Ton Kloks, Mingyu Xiao
