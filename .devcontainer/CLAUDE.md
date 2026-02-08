# .devcontainer/CLAUDE.md

Environment configuration (local only; CC web uses session-start hook instead).

## Structure

```
.devcontainer/
  local/          # Jörn's Ubuntu desktop (primary)
    devcontainer.json
    Dockerfile
    post-create.sh
    host-devcontainer-rebuild.sh
    host-vscode-tunnel.sh
    worktree-new.sh
    worktree-remove.sh
  scripts/        # Shared scripts
    setup-common.sh
    warmup-cache.sh
```

## Dependencies

For system dependencies: `local/{Dockerfile,post-create.sh}`.
See `{crates,experiments,thesis}/CLAUDE.md` for Rust, Python, LaTeX dependency management.
