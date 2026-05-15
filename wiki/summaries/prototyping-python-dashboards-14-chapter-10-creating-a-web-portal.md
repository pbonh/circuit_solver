---
title: "Prototyping Python Dashboards — Chapter 10: Creating a Web Portal"
type: summary
tags: [web, wordpress, html, css, dashboard, deployment]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt"]
confidence: high
---

## Key Points

- A web portal is the entry point that bundles your dashboards, documentation, blog forums, and brand elements into a coherent project home.
- The simplest portal is a plain HTML page with a welcome header and links to dashboards; placing it at the top of the web tree lets you bolt on `.htaccess`-based access controls covering the whole project.
- WordPress is a more capable portal solution but introduces complexity through themes, plugins, and CSS overrides; it requires MySQL as a prerequisite.
- Portal design goals proposed by the author: uncluttered look, easy access to dashboards from the landing page, visual previews enticing exploration, an obvious docs location, registration and blog area for student courses, and no advertisements.
- The "twentysixteen" WordPress theme had an excessive padding-top spacing in sidebar list items; the chapter walks through diagnosing it with Chrome Developer Tools.
- Chrome Developer Tools workflow: open via three-dots menu → More Tools → Developer Tools; use the Elements panel to drill into the DOM (clicking small triangles for hidden layers); the Styles panel reveals associated CSS; toggle the blue checkbox to test enabling/disabling individual rules in real time.
- WordPress allows custom CSS overrides via the theme's Additional CSS field; setting `.widget li { padding-top: 0em; }` (or similar) overrides theme defaults and produces tighter list spacing.
- Themes typically have intentional limitations that drive paid customization — choose simple stock themes and live with minor quirks unless a fix is straightforward.

## Relevant Concepts

- [[concepts/web-portal]] — the curated landing page for end users.
- [[concepts/dashboard]] — the artifact the portal exposes.
- [[concepts/css]] — used both in custom overrides and as the tool diagnosed via Developer Tools.
- [[concepts/html]] — minimal portal page format.
- [[entities/wordpress]] — blogging/CMS used for the portal.
- [[entities/mysql]] — database backing WordPress.
- [[entities/chrome-developer-tools]] — browser DevTools for inspecting CSS/HTML.
- [[entities/avopsinsight]] — the author's demo portal site at avopsinsight.com.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 10 — Creating a Web Portal
- File path: raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt
- Author: Padraig Houlahan
