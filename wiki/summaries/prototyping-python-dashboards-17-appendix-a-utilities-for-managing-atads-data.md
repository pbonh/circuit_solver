---
title: 'Prototyping Python Dashboards — Appendix A: Utilities for Managing ATADS Data'
type: source
id: source-prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data
kind: derived-summary
tags:
- python
- screen-scraping
- data-cleaning
- csv
- pandas
- automation
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/17-appendix-a-utilities-for-managing-atads-data.txt
---

## Key Points

- The ATADS data refresh workflow centers on a folder (`ATADS_DATA_UTILS`) containing three subfolders: `ATADS_XLS` (downloaded annual Excel files), `ATADS_CSV` (converted annual CSVs), and `APT_CSV` (per-airport CSVs).
- Three core utilities ship in the appendix: `atads_scrape.py` (Selenium screen-scraping driver), `xls2csv.py` (Excel-HTML to CSV converter), and `split_by_apt.py` (annual-to-per-airport splitter).
- `atads_scrape.py` requires Chrome and `chromedriver.exe` (placed in `ATADS_DATA_UTILS`); at the time of writing chromedriver had a bug where it ignored the user-specified download directory and dropped files in Chrome's default downloads folder.
- The pipeline favors simplicity over efficiency: `xls2csv.py` always rebuilds every annual CSV (not just the current year), and `split_by_apt.py` always rebuilds every airport CSV from scratch.
- The documented update procedure for adding 2023 data: set the year in `atads_scrape.py`, run it, rename `WEB-Report-*.xls` to `atads2023.xls`, place it in `ATADS_XLS`, run `xls2csv.py`, then run `split_by_apt.py`; copy the resulting `APT_CSV` into the active dashboard area.
- The update process could be automated with a monthly Unix cron job (requires a virtual desktop for the browser-driven scrape if running headlessly).

## Relevant Concepts

- [[concepts/screen-scraping]] — automated browser-driven data extraction.
- [[concepts/csv]] — the cleaned target format.
- [[concepts/data-cleaning]] — HTML stripping and whitespace fixes during conversion.
- [[entities/pandas]] — library used by `split_by_apt.py` for grouping by airport.
- [[concepts/cron]] — Unix scheduler suggested for automating the refresh.
- [[entities/atads-dataset]] — the FAA dataset the pipeline maintains.
- [[entities/chromedriver]] — the WebDriver implementation driving Chrome.
- [[entities/selenium]] — Python web-driver library backing `atads_scrape.py`.

## Source Metadata

- Source type: book appendix
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: Appendix A — Utilities for Managing ATADS Data
- File path: raw/PrototypingPythonDashboards/_txt/17-appendix-a-utilities-for-managing-atads-data.txt
- Author: Padraig Houlahan
