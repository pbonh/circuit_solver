---
title: WordPress
type: entity
id: entity-wordpress
tags:
- web
- cms
- dashboard
- blogging
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt
---

## Overview

WordPress is an open-source PHP-based content management system (CMS) widely used for blogs, documentation, and project portals. In the book it backs the `avopsinsight.com` portal that houses the ATADS dashboards alongside discussion forums and documentation.

## Characteristics

- Plugin and theme ecosystem covering most extensibility needs.
- Requires a MySQL/MariaDB backend for data storage.
- Themes (e.g., the author's `twentysixteen` choice) control look-and-feel and frequently require CSS overrides.
- "Additional CSS" field per theme lets users override defaults without touching theme files.

## Common Strategies

- Pick a simple stock theme and live with quirks until you have time to customize.
- Diagnose layout issues with Chrome Developer Tools to find the offending CSS rule, then override it via Additional CSS.
- Co-host on the same Ubuntu server as the dashboards, with NGINX routing different paths to WordPress vs. Dash backends.
- Periodic backup of the MySQL database alongside the application files.

## Related Entities

- [[entities/mysql]]
- [[entities/nginx]]
- [[entities/chrome-developer-tools]]
- [[entities/avopsinsight]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
