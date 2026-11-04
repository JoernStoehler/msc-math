# Historical file-viewer implementation — retired

**Retired 1 November 2026:** Jörn selected standard client frontends for agent
work and asked to remove the custom file-viewer surface. The workstation
`msc-math-files.service` unit and its drop-in were removed; no Tailscale `/files`
route was configured. The `review-files` project skill link was removed too.
Use the built-in file/document view in the selected T3, Codex, Claude Code or
ChatGPT client. Keep this source and its historical records; do not start the
custom server or recreate a hosted route without a concrete selected need.

The implementation source is [`scripts/serve-files.py`](../scripts/serve-files.py).
It remains in the repository as historical code.

**Current delivery check, 29 October 2026, 00:31 UTC:** the former `/files`
route below is unavailable. `msc-math-files.service` is disabled and inactive;
the current Tailscale Serve map has no `/files` mount. A request for this
checkout's PDF at the old address returned Desk's HTML shell, despite HTTP
200. Do not use the historical prefix as a working delivery link or restart
the inactive service merely because this document once described it as enabled.

At the time of that check, the active client's file links were preferred, and
temporary review/share workflows were available. Those custom browser routes
are retired; no persistent replacement URL is selected.
The research report remains retained with its source packet in this checkout.
The named-file `review-files` route was exercised in the same run: its browser
endpoint returned `application/pdf` and bytes matching the retained report's
SHA-256. That verifies this temporary delivery route, not restoration of the
old persistent server or access from every personal device.

The remaining sections record old behavior and implementation boundaries.
They are historical, not instructions to launch or deploy the server.

The project tool is [scripts/serve-files.py](../scripts/serve-files.py). It uses
Python's standard HTTP server and installed Pandoc; it has no Python package
dependencies. One server serves multiple checkouts, scratch directories and
artifacts. The server is read-only; HTML runs normally in the browser.
Install Pandoc with `sudo apt install pandoc` on Debian/Ubuntu or
`brew install pandoc` on macOS if it is missing.

## Historical URL rule

The former deployment prefixed an absolute file path with
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

Workbench's two published builds, `~/.codex/artifacts/workbench-resource-2026-10-19`
and `workbench-docs-2026-10-19`, have a narrowly scoped transfer policy. Raw
HTML/JavaScript/CSS can use negotiated gzip, with `Vary: Accept, Accept-Encoding`
and representation-specific ETags/HEAD/304 support. Only recognized Vite/VitePress
content-hash filenames under their `assets/` directories receive browser-private
one-year immutable caching. Symlink aliases remain `no-store`, even when their
targets are hashed assets. HTML, JSON/data, unhashed files and rendered code
views remain `no-store`; other directories retain the original behavior. No root
is added. A bounded in-memory representation cache invalidates when a source
file's stat identity changes; files over 4 MiB retain the original streaming path.

Relative links retain their filesystem meaning. Absolute links within a
document remain absolute web paths; the viewer does not rewrite document
contents. The project dashboards use relative paths to their own checkout.

For dashboard links, resolve `docs/dashboard/index.html` (coordination) or
`docs/dashboard/thesis.html` (thesis) against the intended checkout's absolute
root, then apply the URL rule above. Its data and source links must resolve
within that same checkout; do not link a sibling worktree's page as this task's
current state.

## Historical local-server procedure

Do not launch this custom viewer as part of ordinary agent work. The following
commands are retained only to explain the former interface; code and Git history
remain available if a concrete future need is selected.

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

## Removed workstation service and HTTPS route

The unit and host drop-in were removed on 1 November 2026. The Tailscale map
had no `/files` route; no route command is needed. Earlier deployment and
rollback details remain in Git history.

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
