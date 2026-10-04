# View workstation files in a browser

The project tool is [scripts/serve-files.py](../scripts/serve-files.py). It uses
Python's standard HTTP server and installed Pandoc; it has no Python package
dependencies. One server serves multiple checkouts, scratch directories and
artifacts. The server is read-only; HTML runs normally in the browser.
Install Pandoc with `sudo apt install pandoc` on Debian/Ubuntu or
`brew install pandoc` on macOS if it is missing.

## URL rule

On this workstation, prefix an absolute file path with
`https://joern-pc.tailc5e761.ts.net/files`. Expand `~` and URL-encode special
characters, retaining `/`. For example:

```
/home/joern/.codex/artifacts/example.md
https://joern-pc.tailc5e761.ts.net/files/home/joern/.codex/artifacts/example.md
```

Directories have listings. Markdown renders as HTML with native MathML for
mathematics. JSON, TOML and common code/text files show as escaped text when
opened through browser navigation. `?raw=1` returns the original file;
`?view=1` requests a text view explicitly. HTML, PDF, images, JavaScript and
CSS retain their bytes and media types. JSON fetches return JSON rather than
the navigation view, so dashboards can use the same URL. Files larger than
2 MiB offer raw access instead of rendering; raw files stream through Python's
HTTP server. Byte-range support is not implemented.

Relative links retain their filesystem meaning. Absolute links within a
document remain absolute web paths; the viewer does not rewrite document
contents. The project dashboards use relative paths to their own checkout.

## Run locally

```bash
python3 scripts/serve-files.py
# Open http://127.0.0.1:8770/files/
```

Defaults expose the executing script's checkout, `/workspaces/msc-math`,
`~/.codex/worktrees`, `~/.codex/artifacts`, `/tmp/msc-math` and `/data/msc-math`.
Repeat `--root` to select a different set of directories. Symlink targets must
remain inside that set. A whole-machine view is available with `--root /`,
covering files readable by the server user; it is not the default.

```bash
python3 scripts/serve-files.py --root /tmp/msc-math --root ~/.codex/artifacts
```

The default listener is loopback, port 8770. `--host` and `--port` select another
binding. To move machines, run the script from the copied checkout, select
appropriate roots and replace the workstation hostname in the URL convention.

## Workstation service and HTTPS

**4 October evening repair:** the viewer now runs from main as an enabled
persistent systemd user service. Live HTTPS file checks returned HTTP 200.
The consolidation-time outage remains a historical observation.

The workstation unit is `~/.config/systemd/user/msc-math-files.service`, with
`WorkingDirectory=/workspaces/msc-math`,
`ExecStart=/usr/bin/python3 /workspaces/msc-math/scripts/serve-files.py`,
`Restart=on-failure` and `WantedBy=default.target`. It starts with the user
manager after reboot. The existing Tailscale Serve `/files` route is unchanged.

```bash
systemctl --user enable --now msc-math-files
systemctl --user restart msc-math-files
```

Tailscale removes the mount prefix before proxying; the backend URL's `/files`
restores it. This adds one route without replacing the existing `/` and
`/gateway` services. Host networking remains host configuration; the repository
owns the viewer code. Inspect and stop only this service/route with:

```bash
systemctl --user status msc-math-files
tailscale serve status
systemctl --user stop msc-math-files
tailscale serve --set-path=/files http://127.0.0.1:8770/files off
```

After code changes, restart with `systemctl --user restart msc-math-files`.
The existing `review-files` skill remains available for expiring, explicitly
registered file shares.

## Dashboard freshness and checks

Coordination state and evidence links resolve in the dashboard's own checkout.
On HTTPS (or localhost), the browser uses Web Crypto to compare cited source
bytes with `docs/dashboard/thesis-sources.json`. Missing sources or unavailable
Web Crypto produce an explicit unknown freshness status. The manifest's
editorial reconciliation flag is preserved; matching hashes do not establish
scientific adequacy. The old dedicated dashboard server keeps its own hash
endpoint for compatibility. This change does not refresh the scientific baseline.

```bash
python3 scripts/test_serve_files.py
python3 scripts/test_project_dashboard.py
```

The viewer tests exercise Markdown/config rendering, JSON navigation versus
fetch, unchanged assets and media types, distinct checkout paths, escaped names,
root boundaries and file-based source change detection. Source-check tests use
Node when installed. Browser layout/device access needs a separate device check.
