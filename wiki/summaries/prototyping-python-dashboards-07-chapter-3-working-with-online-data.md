---
title: 'Prototyping Python Dashboards — Chapter 3: Working with Online Data'
type: source
id: source-prototyping-python-dashboards-07-chapter-3-working-with-online-data
kind: derived-summary
tags:
- python
- screen-scraping
- data-cleaning
- csv
- pandas
- web
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
---

## Key Points

- Real-world data rarely arrives as a clean CSV; the FAA's ATADS dataset is downloadable only in HTML, Word, or Excel formats through a web form requiring multi-step configuration.
- ATADS tracks 500+ US airports across multiple categories (local/itinerant, civil/military, air carrier/air taxi/general aviation, IFR/VFR), with ~170 MB per year and ~20 years of data.
- HTML-encoded numeric cells (e.g., `<td nowrap align=right>1,036</td>` for the value 1036) waste storage and complicate parsing; converting to pure CSV cuts file size significantly.
- Screen scraping uses Selenium and ChromeDriver to automate browser navigation; the author uses Chrome's Developer Tools to identify form field names (e.g., `fm_r` for starting month) by hovering elements until the targeted region highlights.
- Example Selenium call: `driver.find_element(By.XPATH,"//select[@name='fm_r']/option[text()='Jan']").click()` selects a value and clicks.
- The author downloads ATADS data one year at a time (avoiding download timeouts) and stores XLS files in an `ATADS_XLS` folder, renaming each to `atadsYYYY.xls`.
- `xls2csv.py` is a small OOD utility that reads the HTML-encoded Excel file line by line, skips style/header rows, extracts cells between `<td>...</td>` tags, removes embedded commas in numbers (via Python `re` regex), and trims trailing whitespace from three-letter airport codes (`re.sub(r'([A-Z]{3})([ ])', r'\1', line0, count=1)`).
- A second pass (`split_by_apt.py`) uses pandas to read each year's CSV, identify unique airport IDs in column 2, and append each airport's rows to a per-airport CSV in an `APT_CSV` folder; each airport file is under ~1 MB versus ~300 MB for the full dataset, making the dashboard's per-session load much lighter.
- The pipeline is designed for simplicity over efficiency: the per-airport directory is rebuilt from scratch each refresh rather than appending incrementally, eliminating ambiguity about overlapping current-year data.

## Relevant Concepts

- [[concepts/screen-scraping]] — programmatic navigation of websites to extract data.
- [[concepts/csv]] — the cleaned tabular format the pipeline targets.
- [[concepts/regular-expression]] — Python `re` library used for cell cleanup.
- [[concepts/data-cleaning]] — removing HTML, commas, whitespace from scraped data.
- [[concepts/dataframe]] — pandas structure used to split annual CSVs by airport.
- [[entities/pandas]] — library used to read CSVs and group by airport.
- [[concepts/object-oriented-design]] — `xls2csv` is a class even though it's a small utility.
- [[entities/atads-dataset]] — the FAA dataset the book uses end-to-end.
- [[entities/chromedriver]] — Selenium WebDriver implementation for Chrome.
- [[entities/selenium]] — browser-automation library underlying the scraping code.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 3 — Working with Online Data
- File path: raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
- Author: Padraig Houlahan
