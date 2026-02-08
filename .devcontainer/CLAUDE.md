# .devcontainer/CLAUDE.md

Local devcontainer for Jörn's Ubuntu desktop. Claude Code on the web uses the session-start hook at `.claude/hooks/session-start.sh` instead.

## Files

```
.devcontainer/
  devcontainer.json          # Container config (mounts, env vars, memory limits)
  Dockerfile                 # Image build (deps, toolchains)
  post-create.sh             # Runtime setup after container creation
  setup-common.sh            # Shared setup (npm, gh auth, Claude Code)
  warmup-cache.sh            # Background cache warming (cargo, uv)
  host-devcontainer-rebuild.sh  # Host-side: rebuild image + recreate container
  host-vscode-tunnel.sh      # Host-side: launch VS Code tunnel
  worktree-new.sh            # Create worktree with dep hydration
  worktree-remove.sh         # Safe worktree removal with diagnostics
```

## Dependencies

For system dependencies: `Dockerfile` and `post-create.sh`.
See `{crates,experiments,thesis}/CLAUDE.md` for Rust, Python, LaTeX dependency management.
