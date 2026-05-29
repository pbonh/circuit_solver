---
title: MySQL
type: entity
id: entities/mysql
tags:
- database
- sql
- web
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt
---

## Overview

MySQL is a widely deployed open-source relational database. In the book it backs WordPress, storing posts, users, and configuration for the project's web portal.

## Characteristics

- SQL-based relational model with ACID guarantees on InnoDB.
- Free Community Edition available alongside commercial editions.
- Standard Ubuntu install via `apt`.
- Listens on TCP port 3306 by default.

## Common Strategies

- Install on the same host as WordPress for simplicity in small deployments.
- Restrict network exposure: bind to localhost only and rely on the WordPress PHP process for access.
- Back up regularly via `mysqldump` or snapshots; restore-tested backups are essential.
- Use separate databases per application when colocated.

## Related Entities

- [[entities/wordpress]]
- [[entities/ubuntu]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
