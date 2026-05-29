---
title: Beautiful Soup
type: entity
id: entities/beautiful-soup
tags:
- python
- html
- parsing
- library
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/08-chapter-5-data-gathering-and-cleaning.txt
---

## Overview

Beautiful Soup is a Python library for pulling data out of HTML and XML files. The chapter uses it to navigate a parsed document tree, extract tag attributes (`href`, `id`, `class`), and pull out all `<a>` anchor URLs from web pages.

## Characteristics

- Wraps an underlying parser (`html.parser`, `lxml`, `html5lib`) with a uniform API
- Navigates with `.title`, `.head`, `.body`, `.a`, `.p['class']`
- Searches with `find`, `find_all`, and `findAll`
- Renders prettified HTML with `.prettify()` and extracts text with `.get_text()`

## Common Strategies

- Use `find_all('a')` to harvest links from a page
- Combine with `requests` or `urllib.request` to fetch pages first
- Choose `lxml` for speed and robustness when scraping at scale
- Fall back to regex only when Beautiful Soup is insufficient

## Related Entities

- [[entities/pandas]]
- [[entities/numpy]]

## Sources

- [[summaries/data-analysis-visualizations-python-08-chapter-5-data-gathering-and-cleaning]]
