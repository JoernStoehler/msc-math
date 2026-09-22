# Candidate sbx environment

This directory contains the reviewable, repository-owned part of the MSc Math
sandbox. It is a candidate only: no image has been built, pulled, created, or
started from it.

The environment file is [sbxenv.yaml](sbxenv.yaml) beside this document, so a
fresh checkout carries the inspectable environment definition with it. It
contains host paths and must be reviewed when used on another host. The sbx
commands below are intentionally direct rather than hidden behind a wrapper.

## Phase 1 contract

Phase 1 deliberately optimizes for low-friction host/VM cooperation. These
paths are direct read-write host mounts:

| Host path | Guest path | Purpose |
| --- | --- | --- |
| `/workspaces/msc-math` | `/workspaces/msc-math` | Main checkout and normal active work |
| `/worktrees/msc-math` | `/worktrees/msc-math` | Long-lived sibling worktrees |
| `/data/msc-math` | `/data/msc-math` | Durable local project data |
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

## Image rebuild cache

The Dockerfile is ordered by expected change frequency and build cost:

```text
base/apt -> Miniforge -> Sage environment -> rustup -> Rust toolchain -> uv -> Sage launcher link
```

The repository is never copied into the image, so ordinary source edits do not
invalidate the image build. The large Sage layer remains cached when the Rust
toolchain or `uv` layer changes. Changing only the Sage launcher link rebuilds only
the final launcher-link layer. Changing Sage itself redoes the later toolchain layers;
that is accepted because Sage changes are expected to be rare and the Sage
solve is the expensive layer being changed anyway. BuildKit cache mounts retain
the Miniforge package downloads and Rustup downloads across such layer rebuilds
without putting those caches into the final image.

## Caches and sessions

The profile puts the VM's Cargo home and target directory under
`/tmp/msc-math/cache/vm/`. Host-side builds can use a separate
`/tmp/msc-math/cache/host/` tree. This keeps cache reuse possible without
making the VM depend on the old 9.4 GB environment tree.

While the sandbox is running, its canonical rollout files are under
`/home/agent/.codex/sessions`. Before stopping it for removal or recreation,
run the existing importer explicitly. It validates and merges those rollout
files into the host's `~/.codex/sessions` store, so the normal host session
list and analysis tools see host and VM sessions together. There is no separate
raw archive or host/VM distinction in the normal session store. No session
export is declared as an sbx host lifecycle hook, so the host-side action
remains visible and reviewable rather than being hidden in a file an agent can
edit.

## Deliberately unresolved review points

- The profile has no secrets, credential bindings, MCP servers, ports, or
  host lifecycle commands yet. Those should be added only after deciding which
  narrowly scoped credentials are actually needed.
- Network policy is not encoded here. The useful decision is which ordinary
  development traffic should work by default, not whether to reproduce a
  Docker-style deny-all policy that would make package installation and normal
  research work unnecessarily difficult.
- `Dockerfile` is a candidate tool image. Its base template provides the sbx
  Codex capability. The image's baked Codex version is only the bootstrap
  version; update Codex explicitly after each fresh creation using the command
  below. It also bakes the repository's pinned Rust toolchain and Sage 10.9;
  this is intentionally a substantial first build, after which recreation can
  reuse the image without carrying the old `.local-environments/` tree.

## Intended lifecycle

The intended sequence is:

```text
review files -> plan -> create -> run/exec -> export sessions -> remove
```

From the host:

```bash
sbx env plan /workspaces/msc-math/sandbox/sbxenv.yaml
sbx env create /workspaces/msc-math/sandbox/sbxenv.yaml
sbx env exec /workspaces/msc-math/sandbox/sbxenv.yaml -- sh -lc \
  'curl -fsSL https://chatgpt.com/codex/install.sh | CODEX_NON_INTERACTIVE=1 sh'
sbx env run /workspaces/msc-math/sandbox/sbxenv.yaml
sbx env exec /workspaces/msc-math/sandbox/sbxenv.yaml -- cargo check --workspace
sbx stop codex-msc-math
~/.dotfiles/scripts/import-sbx-codex-sessions.sh codex-msc-math
sbx env rm /workspaces/msc-math/sandbox/sbxenv.yaml
```

`create` and `run` are separate from image construction. Recreating the
sandbox should be cheap and should not be required merely to preserve source,
worktrees, data, scratch, or exported sessions because those are host paths in
phase 1.
