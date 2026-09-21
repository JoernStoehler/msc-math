# Candidate sbx environment

This directory contains the reviewable, repository-owned part of the MSc Math
sandbox. It is a candidate only: no image has been built, pulled, created, or
started from it.

The host-specific environment file lives in
`~/.dotfiles/sandboxes/msc-math.sbxenv.yaml`. Keeping that file out of this
repository avoids putting host paths, host resource sizing, and future secret
references into a checkout that agents can modify. The wrapper at
`~/.dotfiles/scripts/msc-math-sbx` is the intended entry point.

## Phase 1 contract

Phase 1 deliberately optimizes for low-friction host/VM cooperation. These
paths are direct read-write host mounts:

| Host path | Guest path | Purpose |
| --- | --- | --- |
| `/workspaces/msc-math` | `/workspaces/msc-math` | Main checkout and normal active work |
| `/worktrees/msc-math` | `/worktrees/msc-math` | Long-lived sibling worktrees |
| `/data/msc-math` | `/data/msc-math` | Durable local data and exported session logs |
| `/tmp/msc-math` | `/tmp/msc-math` | Disposable scratch and host/VM-shared caches |

This phase accepts that host-side execution and VM-side execution can both
encounter project files, including Git hooks. The practical mitigation is
operational: do not execute project-controlled material on the host unless that
is intentional. A later VM-only phase can replace the active checkout and
worktrees with VM-private volumes or clones; this candidate does not pretend to
solve that problem.

The project guidance remains authoritative: long-lived worktrees belong below
`/worktrees`, disposable scratch below `/tmp`, and large retained data below
`/data` rather than in Git. The old `.local-environments/` tree is not mounted
or copied into the image.

## Caches and sessions

The profile puts the VM's Cargo home and target directory under
`/tmp/msc-math/cache/vm/`. Host-side builds can use a separate
`/tmp/msc-math/cache/host/` tree. This keeps cache reuse possible without
making the VM depend on the old 9.4 GB environment tree.

The wrapper's `export-sessions` action is intentionally explicit. Before a
sandbox is removed, it imports canonical rollout files into the host Codex
session store and archives the raw JSONL/JSONL.zst files under
`/data/msc-math/sessions/raw/<sandbox-name>/`. The archive is data for later
analysis, not an image layer and not a Git artifact. No session export is
declared as an sbx host lifecycle hook, so the host-side action remains visible
and reviewable rather than being hidden in a file an agent can edit.

## Deliberately unresolved review points

- The profile has no secrets, credential bindings, MCP servers, ports, or
  host lifecycle commands yet. Those should be added only after deciding which
  narrowly scoped credentials are actually needed.
- Network policy is not encoded here. The useful decision is which ordinary
  development traffic should work by default, not whether to reproduce a
  Docker-style deny-all policy that would make package installation and normal
  research work unnecessarily difficult.
- `Dockerfile` is a candidate tool image. Its base template provides the sbx
  Codex capability, but the Codex CLI version supplied by that template is not
  treated as a project contract. Pinning/updating it, and choosing the Sage
  installation mechanism, should happen during review before a build.

## Intended lifecycle

The intended sequence is:

```text
review files -> sbx plan -> create -> run/exec -> export-sessions -> remove
```

`create` and `run` are separate from image construction. Recreating the
sandbox should be cheap and should not be required merely to preserve source,
worktrees, data, scratch, or exported sessions because those are host paths in
phase 1.
