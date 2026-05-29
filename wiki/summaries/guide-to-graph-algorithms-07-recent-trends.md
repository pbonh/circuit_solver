---
title: Guide to Graph Algorithms — Recent Trends (Chapter 4)
type: source
id: source-guide-to-graph-algorithms-07-recent-trends
kind: derived-summary
tags:
- graph
- algorithm
- advanced
- emerging
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
---

## Key Points

- The chapter uses treewidth as a "chassis" to introduce many recent developments in graph algorithms. A triangulation (chordal embedding) of G is a chordal supergraph on the same vertex set; treewidth tw(G) = min{ω(H) - 1 : H is a chordal embedding}. A chordal graph is characterized by: every minimal separator is a clique, or equivalently, it has a perfect elimination order, or equivalently, it has a clique tree.
- The Seymour-Thomas theorem (1993): tw(G) + 1 = bramble number b(G); the chapter sketches the proof for chordal graphs and gives the tree-decomposition (T, {X_i}) characterization equivalent to treewidth k. Bodlaender (1996) gives a linear-time algorithm for tree-decompositions of bounded width; nice tree-decompositions facilitate dynamic-programming algorithms (e.g. Steiner tree on bounded treewidth in O(k · B_{2k+1} · n) using Bell numbers).
- Treewidth of circle graphs is computed in O(n^3) via dynamic programming on plane triangulations of a polygon; treewidth of planar graphs is open, but Seymour-Thomas give a 3/2-approximation in O(n^4) via carving width and antipodalities (tilts → slopes → antipodalities on the dual G*).
- Carvings, bond carvings, and the p-carving width framework allow proving Theorem 4.33: for a connected planar graph with all p(δ(x)) < k, p-carving width is ≥ k iff there is an antipodality of p-range ≥ k. The proof proceeds via Robertson-Seymour's "Graph Minors X" bias/tilt machinery and uses an O(m^2)-algorithm to check antipodality through round sets.
- Tree-degree τ(G) of a graph: smallest k such that G is the edge-intersection graph of subtrees of a tree of maximum degree ≤ k. τ ≤ 2 iff G is an interval graph; τ ≤ 3 iff G is chordal. For bounded τ, treewidth is computable in polynomial time because the number of minimal separators is O(m·2^τ).
- Modular decomposition: a module is a set X ⊆ V such that every vertex outside X is adjacent to all or none of X. Tedder-Corneil-Habib-Paul give a linear-time algorithm computing the modular decomposition tree using BFS layers, three procedures (refinement, promotion, assembly), and a factorizing-permutation invariant. Probe permutation graphs (vertices partitioned into probes and nonprobes) are recognized similarly.
- Rankwidth: a graph has rankwidth ≤ k if a carving exists where every cut matrix has GF[2] rank ≤ k. Distance-hereditary graphs (no induced house/hole/domino/gem) are exactly the graphs of rankwidth ≤ 1. Hliněný-Oum (2008) give a cubic FPT algorithm for rankwidth ≤ k. Perfect graphs satisfy χ = ω on all induced subgraphs, and the Strong Perfect Graph Theorem characterizes them as the graphs with no odd hole or odd antihole.
- χ-boundedness: a class is χ-bounded if χ ≤ f(ω) for some function f. Lemma 4.72 shows the class of graphs of rankwidth ≤ k is χ-bounded with f(s) = 2^(k·s)·3^(s-1). Bonamy-Pilipczuk (2020) prove graphs of bounded diversity are polynomially χ-bounded via Kruskalian decompositions and Colcombet's combinatorial tree theorem.
- Clustered coloring: Van den Heuvel-Wood (2018) show every K_t-minor-free graph has a (2t-2)-clustered coloring with cluster size ⌈(t-2)/2⌉. The proof uses BFS-trees with few leaves, bandwidth ≤ k-1 for minimal induced subgraphs spanning a set, and a "connected partition" greedily built.
- Well-quasi-orders: Higman's lemma (finite alphabet A: A* is well-quasi-ordered by subsequence). Kruskal's theorem extends to labeled rooted trees. The gap-embedding theorem (with edge labels in a totally ordered set) is used pervasively. Robertson-Seymour: the class of all graphs is well-quasi-ordered by the minor relation (Graph Minor Theorem); every minor-closed class has a finite obstruction set (e.g. {K5, K3,3} for planar graphs).
- Threshold graphs and threshold-width: a graph is threshold iff every induced subgraph has an isolated or universal vertex iff it has no induced P4/C4/2K2. Threshold-width τ(G) is the minimal k such that G has k independent sets witnessing a threshold embedding. τ ≤ k is FPT (via Higman's lemma and forbidden subgraph characterization); rankwidth ≤ 2^τ. The chapter also presents an explicit O(n^2) algorithm via "probe-universal sets" and "k-probe modules".
- Black-and-white coloring (placing b black and w white queens / vertices such that no black is adjacent to white) is in P on graphs of bounded treewidth and on cographs (O(n^3)), but NP-complete on splitgraphs.
- k-cographs: parameterized cographs where leaves of the decomposition tree carry labels in [k] and internal nodes are labeled by symmetric Boolean k×k-matrices (1-cographs = ordinary cographs). Each class C(k) has a finite obstruction set (via Kruskal). Recognition is FPT (O(n^3)).
- Minors and the Graph Minor Theorem: H is a minor of G iff V(G) partitions into connected pieces V_1,...,V_h such that adjacencies of H lift to edges between pieces. Every minor-closed class has a finite obstruction set; planar graphs are characterized by obstruction set {K5, K3,3}.
- General partition graphs: graphs admitting a clique-cover such that every maximal independent set hits every clique. They satisfy the triangle condition; for minor-closed classes with ω ≤ k, polynomial recognition follows.
- Tournaments and oriented trees: every tournament has a winning probability distribution (Fisher-Ryan, proved via Farkas's lemma). El Sahili: every tournament with 3(n-1) vertices contains every oriented tree with n vertices. Sumner's conjecture (2(n-1)) holds for large enough n. The chapter develops median orders, well-rooted trees, branchings, and M-embeddings.
- Chudnovsky-Seymour: tournaments are well-quasi-ordered by strong immersion. Proof goes through linked layouts of cutwidth k, gap sequences, codewords, and Higman/Kruskal-style induction. Bousquet-Lochet-Thomassé (2017) settle the Erdős-Sands-Sauer-Woodrow conjecture: a complete multi-digraph whose arcs union to k quasi-orders has γ(T) = O(k^(k+2)·ln(2k)).
- Liu-Muzi: digraphs without k-alternating paths are well-quasi-ordered by strong immersions. Subcubic graphs are also well-quasi-ordered by strong immersions. Liu-Thomas/Robertson-conjecture-I shows topological-minor well-quasi-ordering holds modulo Robertson chains.
- Asteroidal sets and AT-free graphs: An asteroidal triple is three vertices each in a component of G - N[other]. Interval graphs are chordal AT-free. AT-free graphs have dominating pairs and are χ-bounded (Kierstead-Penrice). α is computable in O(n^4) for AT-free; bandwidth is 6-approximable in linear time via spanning caterpillars at distance ≤ 4. AT-free orders correspond to convex geometries / antimatroids.
- Sensitivity Conjecture (Huang 2019): for every Boolean function f, s(f) ≤ bs(f) ≤ s(f)^4. Proof goes through Cauchy's interlace lemma applied to a {0,-1,+1}-matrix A_n with eigenvalues ±√n of equal multiplicity, giving the hypercube theorem (any induced subgraph of Q_n with > 2^(n-1) vertices has Δ ≥ √n), combined with the Gotsman-Linial equivalence theorem and Tal's bs(f) ≤ δ(f)^2.
- Homomorphisms and retracts: H is a retract of G if there are homomorphisms ρ: G → H, γ: H → G with ρ∘γ = id_H. Retract problem is linear-time on threshold graphs but NP-complete on cographs (via reduction from 3-partition).
- Products: tensor (categorical) product G × H, Cartesian product G □ H. Hedetniemi's conjecture (χ(G×H) = min{χ(G), χ(H)}) was refuted by Shitov (2019), but holds for perfect graphs. Independence ratio and tensor capacity Θ(G) = lim r(G^k) = a*(G) (Tóth) yield polynomial-time tensor capacity for cographs. Vizing's conjecture γ(G□H) ≥ γ(G)·γ(H) is proved for chordal graphs by Aharoni-Szabó. θ_e(K_n × K_n) = n(n-1) iff a projective plane of order n exists.
- Outerplanar and k-outerplanar graphs: outerplanar graphs have treewidth ≤ 2 with obstruction set {K4, K_{2,3}}; k-outerplanar graphs have treewidth ≤ 3k - 1 (Bodlaender). Baker's method, combined with Courcelle's MS2 theorem, gives PTAS schemes for many planar-graph problems (e.g. independent set within factor k/(k+1)).
- Graph isomorphism: noted as out of scope but acknowledged with Babai's quasipolynomial algorithm; reader directed to Grohe-Neuen survey.

## Relevant Concepts

- [[concepts/treewidth]] — the chassis parameter of the chapter
- [[concepts/chordal-graph]] — graphs with no induced cycle longer than 3
- [[concepts/triangulation]] — chordal embedding of a graph
- [[concepts/clique-tree]] — tree-of-cliques representation of a chordal graph
- [[concepts/simplicial-vertex]] — vertex whose neighborhood is a clique
- [[concepts/bramble]] — set of pairwise-touching connected subsets
- [[concepts/tree-decomposition]] — width-k partition into bags forming a tree
- [[concepts/steiner-tree]] — minimum connected subgraph spanning a set of terminals
- [[concepts/circle-graph]] — intersection graph of chords of a circle
- [[concepts/carving]] — maximal cross-free family of subsets
- [[concepts/carving-width]] — minimum width over all carvings
- [[concepts/antipodality]] — Robertson-Seymour structure for planar p-carving width
- [[concepts/tree-degree]] — τ(G), bounded edge-intersection representation in trees
- [[concepts/interval-graph]] — intersection graph of intervals on a line
- [[concepts/edge-clique-cover]] — covering edges with cliques
- [[concepts/modular-decomposition]] — decomposition tree by modules
- [[concepts/module]] — uniformly-neighbored vertex set
- [[concepts/rankwidth]] — GF[2]-rank-based width parameter
- [[concepts/distance-hereditary-graph]] — chordless paths preserve distance; rankwidth ≤ 1
- [[concepts/perfect-graph]] — χ = ω on every induced subgraph
- [[concepts/chi-boundedness]] — class with χ ≤ f(ω)
- [[concepts/clustered-coloring]] — coloring with bounded monochromatic component size
- [[concepts/well-quasi-order]] — order with no infinite antichains and no infinite decreasing chains
- [[concepts/higmans-lemma]] — A* is wqo under subsequence
- [[concepts/kruskal-theorem]] — labeled trees are wqo under embedding
- [[concepts/graph-minor-theorem]] — Robertson-Seymour wqo by minor
- [[concepts/minor]] — relation closed under deletions and contractions
- [[concepts/threshold-graph]] — graphs with iso/universal vertices in every induced subgraph
- [[concepts/threshold-width]] — k independent sets witnessing a threshold embedding
- [[concepts/k-cograph]] — parameterized cograph hierarchy
- [[concepts/cograph]] — P4-free graph
- [[concepts/general-partition-graph]] — clique cover meeting every maximum independent set
- [[concepts/tournament]] — orientation of a complete graph
- [[concepts/median-order]] — feedback-arc-set-minimizing vertex order
- [[concepts/immersion]] — relation via edge-disjoint path embeddings
- [[concepts/strong-immersion]] — immersion preserving non-incidence
- [[concepts/topological-minor]] — subgraph that is a subdivision of H
- [[concepts/asteroidal-triple]] — three vertices each in a component of G - N[other]
- [[concepts/at-free-graph]] — no asteroidal triple
- [[concepts/dominating-pair]] — pair with every path between them dominating
- [[concepts/antimatroid]] — convex geometry on graphs / posets
- [[concepts/sensitivity]] — Boolean function complexity measure
- [[concepts/block-sensitivity]] — bs(f), generalized sensitivity
- [[concepts/hypercube]] — n-dimensional cube graph Q_n
- [[concepts/cauchy-interlace-lemma]] — eigenvalue interlacing for principal submatrices
- [[concepts/homomorphism]] — edge-preserving vertex map
- [[concepts/retract]] — homomorphism pair with ρ∘γ = id
- [[concepts/tensor-product]] — categorical product of graphs
- [[concepts/cartesian-product]] — Cartesian product of graphs (□)
- [[concepts/hedetniemi-conjecture]] — refuted χ(G×H) conjecture
- [[concepts/vizings-conjecture]] — γ(G□H) ≥ γ(G)·γ(H)
- [[concepts/tensor-capacity]] — limit of independence ratio under tensor powers
- [[concepts/outerplanar-graph]] — planar graph with all vertices on outer face
- [[concepts/k-outerplanar-graph]] — k layers of outerplanarity
- [[concepts/courcelle-theorem]] — MS2 in linear time on bounded treewidth
- [[concepts/bakers-method]] — PTAS for planar-graph optimization via layer decomposition
- [[entities/robertson-seymour-graph-minors]] — long sequence of papers proving the Graph Minor Theorem

## Source Metadata

- Source type: book chapter
- Book title: A Guide to Graph Algorithms
- Chapter 4: Recent Trends
- File path: raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
- Authors: Ton Kloks, Mingyu Xiao
