---
title: AVOPSinsight
type: entity
id: entity-avopsinsight
tags:
- web
- dashboard
- aviation
- portal
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt
---

## Overview

AVOPSinsight (avopsinsight.com) is the author's WordPress-based web portal that hosts the ATADS dashboards developed in *Prototyping Python Dashboards for Scientists and Engineers*. It serves as a documentation hub, blog forum, and entry point for students and aviation professionals exploring the dashboards.

## Characteristics

- WordPress instance running on the same Ubuntu server as the Dash backends.
- Uses the `twentysixteen` theme with CSS overrides applied via the Additional CSS field.
- Houses links to dashboards, chart previews, documentation, and a registration/blog area.
- No advertising; intended primarily for student-course support.

## Common Strategies

- Centralize project access through the portal rather than direct dashboard URLs.
- Diagnose theme quirks with Chrome Developer Tools and patch via Additional CSS.
- Keep the portal as the visible "front door" while NGINX routes specific URL prefixes to underlying Dash apps.

## Related Entities

- [[entities/wordpress]]
- [[entities/nginx]]
- [[entities/atads-dataset]]
- [[entities/dash]]

## Sources

- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
