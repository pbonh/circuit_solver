---
title: "Matplotlib Interfaces (Stateful vs. Object-Oriented)"
type: concept
tags: [python, visualization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt"]
confidence: high
---

## Definition

Matplotlib exposes two APIs for plotting. The stateful (pyplot) interface mimics MATLAB and operates on an implicit current figure/axes object. The object-oriented interface manipulates explicit `Figure` and `Axes` objects, giving better control over multi-panel layouts and per-element customization.

## How It Works

Stateful: `plt.plot(x, y)`, `plt.xlabel(...)`, `plt.title(...)` — all operate on whatever `plt` currently considers "active". Object-oriented: `fig, ax = plt.subplots(...)`, then `ax.plot(...)`, `ax.set_xlabel(...)`, etc. For multiple subplots, use `fig.add_subplot(rows, cols, position)` to assign axes.

## Key Parameters

- Choice of interface
- `figsize` for figure dimensions
- Subplot grid arrangement
- Per-axes labels, titles, limits

## When To Use

- Stateful: quick one-off plots, REPL exploration
- Object-oriented: publication graphics, multi-panel layouts, embedded plots

## Risks & Pitfalls

- Mixing the two APIs leads to confusing state
- Forgetting which axes is "current" yields wrong customizations
- Memory growth from never closing figures in long-running loops

## Related Concepts

- [[concepts/data-visualization]]
- [[entities/matplotlib]]

## Sources

- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
