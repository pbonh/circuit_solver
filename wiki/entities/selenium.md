---
title: Selenium
type: entity
id: entity-selenium
tags:
- python
- web
- automation
- testing
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
---

## Overview

Selenium is a browser-automation framework with Python (and other) bindings. It drives a real browser via WebDriver to simulate user interactions, used in the book to scrape the FAA ATADS website which lacks a structured download API.

## Characteristics

- Driver-based architecture: separate driver binaries (ChromeDriver, GeckoDriver, etc.) bridge Selenium to specific browsers.
- Element location strategies: `By.XPATH`, `By.CSS_SELECTOR`, `By.NAME`, `By.ID`, `By.LINK_TEXT`.
- Supports waits (explicit, implicit) to handle asynchronous page loads.
- Often deployed in headless mode for CI but in this book's example uses a visible browser.

## Common Strategies

- Use Chrome Developer Tools to discover element identifiers (name attributes, XPath) before scripting.
- Combine `find_element` with `.click()` to navigate the FAA form (e.g., selecting `fm_r` for the start-of-range month).
- Run on a desktop or VNC-equipped server because cron-driven scraping requires either a virtual display or a headless mode.
- Pair with explicit waits to avoid race conditions on slow forms.

## Related Entities

- [[entities/chromedriver]]
- [[entities/chrome-developer-tools]]
- [[entities/atads-dataset]]

## Sources

- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
