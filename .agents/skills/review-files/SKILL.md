---
name: review-files
description: Create expiring browser links for Jörn to review specific Markdown, PDF, image, text, or standalone HTML files from the host or a configured Docker sandbox. Use for quick view-only feedback without making Jörn locate the artifact. Do not use when he needs a retained device copy or collaborative editing.
---

# Review Files

Use this skill as a delivery surface: create one link per requested artifact,
collect feedback in the current conversation, and keep review distinct from any
later repository change. Use `share-files` instead when Jörn wants a retained
copy on another device and the current agent is running on `joern-pc`; from a
sandbox, ask a host agent to perform that transfer.

## From a configured Docker sandbox

Run from the repository containing this skill:

```bash
.agents/skills/review-files/scripts/review-open --sandbox FILE...
```

The helper reads the external base URL and sandbox port from the repository's
local Git configuration. It starts ordinary Python's HTTP server over a
temporary directory containing random-named links to the requested files and
prints one URL per file. Return those URLs to Jörn. The server stops after eight
hours; a new request replaces the previous sandbox review session.

The temporary directory exposes only the named files, not their source
directories. A review request authorizes presenting those artifacts, not other
readable sandbox content. If setup is missing, return the helper's concrete
error; a host agent can use the setup in dotfiles `INSTALL.md`. Do not give the
sandbox Taildrop credentials or mount the host Tailscale socket.

## From joern-pc

Start the eight-hour review server with:

```bash
scripts/review-open --print-only FILE...
```

Resolve `scripts/` relative to this `SKILL.md`. The command prints one URL and
absolute path per line. It binds only to `joern-pc`'s Tailscale IPv4 address
when Tailscale is running, so the same unguessable URL works from the host and
other permitted tailnet devices. Pass `--local` to restrict it to `127.0.0.1`.

When Chrome control is available or explicitly named, open every returned URL
in Chrome and keep each tab as a user-facing deliverable. When Jörn is reviewing
on another tailnet device, return the URL in the conversation instead. Otherwise
omit `--print-only`; the script asks the systemd user manager to open the URLs
in the host's desktop Chrome session.

Markdown opens rendered at a 16 px body size. Its **Raw source** link switches
the same tab to the exact source representation; use it when YAML frontmatter,
line wrapping, fences, or other Markdown syntax is part of the review. Reload
after the underlying file changes. PDF, PNG, and standalone HTML are served
inline using their detected media types.

The host server exposes only the files passed to that invocation and creates no
review copies. Do not replace this with a broad directory server, cloud upload,
or format conversion merely to deliver the artifact. Treat standalone HTML as
active content rather than a sanitized document.

Ask Jörn for the judgment that the artifact needs and estimate the review
effort. Ordinary feedback belongs in the conversation. When detailed Markdown
changes are easier in Micro, let Jörn edit the actual source and read it only
after he says the edit is saved or complete; the browser is not an editor or a
comment store.
