---
title: "Prototyping Python Dashboards — Chapter 8: Deploying Your Project As a UNIX Service"
type: summary
tags: [python, deployment, systemd, nginx, gunicorn, flask, dash, web, unix, security]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt"]
confidence: high
---

## Key Points

- Systemd services are managed via `systemctl <action> <service>` with actions `start`, `stop`, `enable`, `disable`, and `status`; enabled services start automatically on reboot.
- A service is configured by adding a unit file (e.g., `hwapp.service`) under `/etc/systemd/system` with `[Unit]`, `[Service]`, and `[Install]` blocks; `ExecStart` typically invokes `gunicorn` against the venv-local binary.
- NGINX is installed via `sudo apt install nginx` and acts as a reverse proxy in front of GUNICORN; add server blocks under `/etc/nginx/sites-available` and symlink them into `sites-enabled`.
- Communication between NGINX and GUNICORN can use TCP ports (e.g., 5000) or Unix sockets (e.g., `app.sock` in the project directory); sockets avoid port conflicts and are preferred for production.
- A minimal NGINX server block uses `proxy_pass http://unix:/path/to/app.sock;` to route a location to the socket; if changing `location / {}` to `location /hello {}`, the proxy_pass URL must end with `:/` to fix routing.
- For a Dash dashboard, `app.py` must expose a Flask instance: `server = Flask(__name__); app = dash.Dash(__name__, server=server)`; `wsgi.py` then imports `server` (not `app`) and `server.run()`.
- For Dash, the author keeps a TCP port (5000) rather than a Unix socket because Dash is fussier with sockets.
- A single NGINX server block can route multiple services by URL prefix: `http://A.B.C.D/hello` to the Hello socket and `http://A.B.C.D/atads` to the ATADS Dash app at port 5000.
- Security hardening: switch HTTP to HTTPS via Let's Encrypt; restrict firewall (ufw) to SSH/HTTPS/DNS/FTP; sit behind an institutional firewall if available; install Fail2ban to monitor logs and ban IPs after repeated failures; avoid hosting unneeded services like email; maintain server images for fast disaster recovery.
- The author notes thousands of attack attempts per day on a fresh Internet-facing droplet and warns blocking individual IPs is not scalable.

## Relevant Concepts

- [[concepts/systemd-service]] — Unix mechanism for managing long-running daemons.
- [[concepts/reverse-proxy]] — NGINX role sitting in front of GUNICORN.
- [[concepts/wsgi]] — interface used by gunicorn to call the Python app.
- [[concepts/dashboard]] — the artifact being deployed.
- [[entities/nginx]] — public-facing web server with proxy/static-file roles.
- [[entities/gunicorn]] — WSGI server hosting the Flask/Dash app.
- [[entities/flask]] — provides the `server` instance Dash wraps.
- [[entities/dash]] — Dash app declared via `dash.Dash(server=server)`.
- [[entities/systemd]] — service manager handling service unit files.
- [[entities/letsencrypt]] — free HTTPS certificate provider.
- [[entities/fail2ban]] — log-monitoring intrusion-prevention tool.
- [[entities/ufw]] — Ubuntu's firewall used to restrict open ports.
- [[entities/ubuntu]] — host operating system.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 8 — Deploying Your Project As a UNIX Service
- File path: raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt
- Author: Padraig Houlahan
