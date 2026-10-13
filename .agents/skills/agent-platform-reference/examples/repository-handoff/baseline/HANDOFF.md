# Handoff to Claude.ai chat/Cowork

Upload `handoff-repository.zip` to a Claude.ai Cowork session, then paste this prompt:

```text
Continue this repository from its current uploaded state. First inspect the project files and the report.html included in the upload. Follow the repository's build/cache guidance: keep generated caches outside tracked source and choose a writable cache from the current runtime environment; do not reuse paths from another machine. Check whether Cargo is available before attempting the Rust build. If it is missing, report that exact prerequisite and continue with work that can be verified without it. Generate or update report.html when appropriate, and make the report inspectable in Cowork if the session supports local preview. Tell me what you verified and what remains blocked; do not imply that unavailable destination checks passed.
```

The ZIP contains the committed project tree plus the generated `report.html`. The fixture's `main` branch was pushed to its local bare `origin`; that path is fixture-local and is not a remote reachable from Cowork, so use the ZIP unless you separately provide an accessible checkout.

## Source-side verification

- Updated the cache policy and build instructions and removed the fixed `/workstations/alex/.cache/cargo/project-alpha` Cargo config.
- `bash -n scripts/build.sh` passed.
- `python3 scripts/report.py` generated `report.html`; checked that it is nonempty and contains `Build status` (98 bytes).
- `git diff --check` passed.
- `bash scripts/build.sh` was attempted with a writable temporary target directory and stopped with exit 127: `cargo: command not found`. Rust build verification therefore remains pending in Cowork.
- Cowork availability, Cargo installation, and local preview/port forwarding cannot be verified from this source fixture.
