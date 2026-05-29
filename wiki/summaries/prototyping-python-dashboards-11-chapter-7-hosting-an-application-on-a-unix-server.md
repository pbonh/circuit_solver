---
title: 'Prototyping Python Dashboards — Chapter 7: Hosting an Application on a UNIX
  Server'
type: source
id: source-prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server
kind: derived-summary
tags:
- python
- deployment
- flask
- gunicorn
- nginx
- wsgi
- web
- unix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
---

## Key Points

- Hosting a dashboard on Ubuntu requires a sequence of tasks: create a Python virtual environment, install Flask/uWSGI/GUNICORN, install and configure NGINX, set up virtual hosts, register the app as a system service, and harden with Fail2ban.
- Python virtual environments isolate project dependencies; created with `python -m venv hwenv`, activated via `source hwenv/bin/activate`, and exited via `deactivate`; an `activate` script overrides PATH and home directory.
- WSGI (Web Server Gateway Interface) is a Python specification allowing web servers to call Python applications — not a server itself.
- Flask is a single-threaded WSGI application/framework, fine for development but unsuitable for multi-user production use.
- GUNICORN is a multi-worker WSGI server that hosts Flask apps efficiently but should not face the public Internet directly.
- uWSGI is another WSGI server (confusingly named like the spec); functionally similar to GUNICORN for this use case.
- NGINX is a public-facing web server that proxies dynamic requests to GUNICORN (which talks WSGI to Flask) and serves static pages directly.
- The simplest Flask hello-world: `app = Flask(__name__)`; `@app.route("/") def hello(): return "Hello World!"`; `app.run(debug=True, host='0.0.0.0')` to bind all interfaces.
- Common port-availability tip: `sudo ufw allow 5000` opens port 5000 through Ubuntu's firewall.
- A `wsgi.py` entry-point file (`from hello import app`) lets GUNICORN/uWSGI find the application; invocation: `gunicorn --bind 0.0.0.0:5000 wsgi:app` or equivalently `hello:app`.
- The chapter ends with the app working via Python, Flask, uWSGI, and GUNICORN — but still requires service-ification and NGINX integration in the next chapter.

## Relevant Concepts

- [[concepts/virtual-environment]] — Python isolation mechanism via `venv`.
- [[concepts/wsgi]] — Python web-server interface spec.
- [[concepts/dashboard]] — the artifact being deployed.
- [[entities/flask]] — Python web framework providing the route decorator.
- [[entities/gunicorn]] — multi-worker WSGI server.
- [[entities/nginx]] — outward-facing web server / reverse proxy.
- [[entities/uwsgi]] — alternative WSGI server.
- [[entities/ubuntu]] — Linux distribution used for the server.
- [[entities/ufw]] — Ubuntu's uncomplicated firewall used to open ports.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 7 — Hosting an Application on a UNIX Server
- File path: raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
- Author: Padraig Houlahan
