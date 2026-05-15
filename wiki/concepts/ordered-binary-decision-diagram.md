---
title: "Ordered Binary Decision Diagram (OBDD)"
type: concept
tags: [graph, digital, boolean, eda, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

An Ordered Binary Decision Diagram (OBDD) is a directed acyclic graph that represents a Boolean function. Non-terminal nodes correspond to input variables (in a fixed order along every path), each with two children high(x) (variable true) and low(x) (variable false). Terminal nodes are 0 or 1.

## How It Works

To evaluate the function on an input, traverse the DAG from the root taking high(x) when x=1 and low(x) when x=0; the leaf reached gives the output. Reduction rules — (1) eliminate a node whose two children are identical, and (2) merge equivalent subtrees — produce a Reduced OBDD (ROBDD), which is canonical for a given variable order. Variable ordering critically determines ROBDD size, which can range from linear to exponential in the number of variables.

## Key Parameters

- Variable ordering.
- Number of variables.
- Number of nodes after reduction.

## When To Use

- Equivalence checking of Boolean functions.
- SAT and model-checking subroutines.
- Symbolic state-space exploration.

## Risks & Pitfalls

- Worst-case exponential size.
- Order-sensitive: dynamic variable reordering is often necessary.
- Less scalable than AIG for very large circuits.

## Related Concepts

- [[concepts/and-inverter-graph]]
- [[concepts/binary-decision-diagram]]
- [[concepts/boolean-satisfiability]]
- [[concepts/directed-acyclic-graph]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
