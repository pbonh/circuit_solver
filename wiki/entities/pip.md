---
title: pip
type: entity
id: entities/pip
tags:
- python
- package-manager
- tool
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/05-chapter-2-the-importance-of-data-visualization-in-business-intelligence.txt
---

## Overview

pip is the standard Python package installer, used to fetch and install libraries from PyPI. The book shows pip commands for searching, installing, upgrading, and listing packages such as Matplotlib, both at the shell and within notebooks.

## Characteristics

- Works with any Python distribution, including Anaconda
- Reads from PyPI by default; supports private indexes
- Handles dependency resolution (improved post-2020)
- Commands: `pip search`, `pip install`, `pip install --upgrade`, `pip list`

## Common Strategies

- Combine pip with virtual environments (venv) to isolate project dependencies
- Pin versions in `requirements.txt` for reproducibility
- Use `pip install -e .` for editable installs of local packages
- Bootstrap missing libraries inside notebooks via try/except install patterns

## Related Entities

- [[entities/anaconda]]
- [[entities/jupyter-notebook]]

## Sources

- [[summaries/data-analysis-visualizations-python-05-chapter-2-the-importance-of-data-visualization-in-business-intelligence]]
