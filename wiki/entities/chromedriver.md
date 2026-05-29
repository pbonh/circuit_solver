---
title: ChromeDriver
type: entity
id: entities/chromedriver
tags:
- web
- automation
- browser
- selenium
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
---

## Overview

ChromeDriver is the WebDriver-protocol implementation that Selenium uses to drive Google Chrome. In the book it is required (along with a matching Chrome version) for the `atads_scrape.py` utility to navigate the FAA ATADS site.

## Characteristics

- Distributed as a single executable; version must match the installed Chrome major version.
- Listens on a local port for WebDriver requests from Selenium clients.
- Honors most ChromeOptions including headless mode and download directory (with a noted bug at the time of book authoring).

## Common Strategies

- Place `chromedriver.exe` next to the scraping script (the book's `ATADS_DATA_UTILS` folder).
- Keep Chrome and ChromeDriver versions aligned; broken pairings produce cryptic Selenium errors.
- At the time of writing, ChromeDriver ignored the user-supplied default download directory — recover the file from Chrome's normal Downloads area.
- Use Selenium's ChromeOptions to configure headless mode, user-data-dir, and proxy settings.

## Related Entities

- [[entities/selenium]]
- [[entities/chrome-developer-tools]]
- [[entities/atads-dataset]]

## Sources

- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
