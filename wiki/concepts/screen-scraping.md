---
title: "Screen Scraping"
type: concept
tags: [python, web, data-extraction]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt"]
confidence: high
---

## Definition

Screen scraping is the programmatic navigation of a website's user interface to extract data that is not exposed through a structured API. It typically drives a real browser via automation (e.g., Selenium + ChromeDriver) to click links, fill forms, and trigger downloads.

## How It Works

A scraping script launches a controlled browser instance, navigates to a URL, locates DOM elements by XPath/CSS/ID selectors, and simulates user interactions (clicks, value selections, form submissions). Element identifiers are typically discovered by hovering candidate rows in Chrome Developer Tools until the targeted region highlights. After actions trigger a server-side download, the script copies the resulting file to a known location for processing.

## Key Parameters

- Selector strategy (XPath, CSS selector, link text, element name)
- Browser driver and version compatibility
- Wait/timeout strategy for asynchronous page loads
- Download directory handling (occasionally buggy in ChromeDriver)

## When To Use

- Public data sources that only offer HTML/Excel/Word downloads (e.g., FAA ATADS)
- Sites without rate-limited APIs
- Repeatable, parameterized data refreshes (yearly or monthly)
- Demonstrations where seeing the browser act is pedagogically useful

## Risks & Pitfalls

- DOM changes silently break selectors; scripts need maintenance
- Sites may detect automation and block headless browsers
- Site terms of service may prohibit scraping
- Download directory bugs (e.g., ChromeDriver ignoring user preferences)
- Headless deployment of a real browser requires a virtual desktop or container

## Related Concepts

- [[concepts/data-cleaning]]
- [[concepts/csv]]
- [[entities/selenium]]
- [[entities/chromedriver]]
- [[entities/chrome-developer-tools]]

## Sources

- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
