# Development environments

Codex work on this repository currently runs in three execution environments:

| Environment | Role |
| --- | --- |
| Host | supported direct execution environment and owner of host-only sandbox operations |
| Docker Sandbox (`sbx`) | intended project environment; migration checks remain incomplete (see below) |
| Codex Cloud | rarely used remote environment for work on a selected repository revision |

An execution environment owns the processes, filesystem view, installed
software, network boundary, and runtime state for a thread. A Codex app-server
runs in that environment and serves the thread.

Codex TUI and the OpenAI extension for VS Code are the supported local clients.
ChatGPT Desktop and Paseo were retired from this host on 2026-09-01. Thread
execution occurs in the selected execution environment; the client therefore
does not identify the execution environment.

This table is a topology inventory, not evidence that every environment is
available to a particular thread. Infer the active environment only from
explicit runtime or operator-provided facts. If those facts are absent, report
the uncertainty before giving environment-specific setup or recovery advice.

## Tools and project contracts

[`INSTALL.md`](../INSTALL.md) is the setup and reproduction entry point,
including Sage's recommended route and known limitations. This document records environment
ownership, access and dated verification.

The app-server advertises the function calls, tools, and MCP tools available to
the thread. That surface can differ with the thread's creation path and
execution environment. Agents use the advertised surface and its schemas; this
repository does not maintain a duplicate inventory or usage manual for it.
`.codex/config.toml` may configure operations, features, or integrations, but
the effective tool surface visible to a thread remains authoritative.

Project sources shared across environments own the task-level toolchain
contracts:

- `rust-toolchain.toml` owns the Rust version; the root `Cargo.lock` owns the
  root-workspace dependency graph, while standalone nested workspaces own their
  adjacent lockfiles;
- Python scripts with PEP 723 metadata run through `uv`;
- `thesis/README.md`, `formal/README.md`, and `crates/README.md` own baseline build and validation commands; and
- domain and producer READMEs own additional tools, commands, and output
  contracts.

Installed packages and executable availability can differ between
environments. Check the commands required by the task in the active environment
rather than inferring them from the client or from another thread.

## Host

The host is a supported execution environment, not merely a control plane for
another environment. It also owns Docker Sandbox creation, authentication,
policy, lifecycle, preservation, and recovery.

The cross-project host runbook is the Docker sandbox operations section of
`/home/joern/.dotfiles/memories/host-estate.md`, outside this repository and
on the host. Read it before changing a sandbox.
Host-local sandbox state is not tracked project configuration.

## Docker Sandbox

From the host, inspect the named sandbox and connect with:

```bash
sbx ls --json
ssh codex-msc-math.sbx
```

VM existence and Herdr visibility are separate checks. `sbx ls --json` proves
only that the sandbox exists; `herdr machine list` must also contain an enabled
entry for `codex-msc-math.sbx` before reporting that the VM is available in
Herdr.

Rechecked on 2026-09-13: sandbox `396e73ec-ab29-45e6-92ce-6a4cd1d54a6a`
started successfully with the expected workspace, Codex 0.153.4 and Cargo
1.94.0. Herdr 0.9.0 is installed in the VM, its default-session server reports
a compatible protocol endpoint, and the host has the enabled Herdr machine
entry `msc-math` for `codex-msc-math.sbx`. The sandbox was left running so the
Herdr client can connect to it.

On 2026-09-14, Micro was normalized to the checksum-verified host 2.0.15
binary in `/home/agent/.local/bin`. All five authored files under
`/home/agent/.config/micro/` were refreshed and hash-checked against the host,
including the terminal clipboard settings. Herdr remains 0.9.0 with Codex
integration v8 and the enabled host machine profile `msc-math`; its working
configuration was not otherwise changed.

Interactive Bash now enters the mounted project. For noninteractive commands,
specify the directory explicitly:

```bash
ssh codex-msc-math.sbx 'cd /workspaces/msc-math && cargo test -p euclidean-polytopes --offline --lib'
```

Verified on 2026-09-05 after in-place repair:

- Codex standalone 0.153.4 uses `gpt-6-astra` by default (medium reasoning).
  A live Astra reply and agent shell calls succeeded through the `sandboxd`
  provider. `codex login status` says "Not logged in", but this provider uses
  the host credential proxy and does not require local OpenAI authentication.
- Rust/Cargo 1.94.0 resolve in interactive and noninteractive Bash. All 24
  Cargo workspace packages resolve offline; the geometry crate's 27 library
  tests passed in the sandbox.
- Herdr 0.8.2 (subsequently updated as recorded above), Micro 2.0.15, Python
  3.12.13, uv 0.12.2, rclone and latexmk
  resolve. Micro has the host theme, Markdown syntax and Ctrl+K comment binding;
  Ctrl+K, text entry and Ctrl+S saved `{>>migration check<<}` over SSH.
- GitHub API authentication returned the expected account, `JoernStoehler`.
- Python works through `uv run --offline --no-project`; the project artifact
  helper can read its local registry. Listing the R2 snapshot directory through
  the `mscmath` remote succeeded. Materializing `combinatorial-cell-widths`
  (11,012,601 bytes) with `--no-link` downloaded and hash-verified its registered
  snapshot without changing project links. Other payloads were not reverified.
- Native web search succeeded in an Astra session after setting
  `web_search = "live"`, `features.standalone_web_search = true`, and
  `model_providers.sandboxd.supports_standalone_web_search = true`. The custom
  provider needs this capability declaration in addition to enabling search.
  Codex currently labels standalone search under development and prints a
  warning; that warning was present in the successful check.
- The unused MCP gateway configuration is disabled: no servers were loaded
  into this sandbox and it returned HTTP 503. Enable it only after loading a
  required server with `sbx mcp load` and checking its tools.

The VM-local files owning these repairs are `/home/agent/.bashrc` (interactive
entry), `/home/agent/.profile` (login setup), `/etc/sandbox-persistent.sh` (Rust
PATH), `/home/agent/.config/micro/`, and `/home/agent/.codex/config.toml`.
They are persistent sandbox state, not files supplied by the Git checkout.

Global skills, documentation and memory are owned by DevOps. Its 2026-09-05
handoff reports the agreed migration installed, with no consumer action needed:

- `/home/agent/.agents/skills`: Docker's shared cross-sandbox mount, a clean
  `agent-skills` checkout at `efcf72a500665734e1c5294856b2c2511d8e7d99`.
- `/home/agent/.agents/memories`: VM-local clean `agent-memory` clone at
  `d9a825fa3cf351aedfb56a8234dd0e8b286ffcb9`.

Updates are manual `git pull --ff-only`. Direct edits to the shared skills
checkout immediately affect other projects; do not use `sbx skills import`
or create another global copy. Project-owned facts and skills remain in this
repository. These installation facts are attributed to the DevOps handoff,
not a separate consumer-side verification.

Sage 10.9 is installed with Miniforge/conda-forge in the ignored shared path
`/workspaces/msc-math/.local-environments/miniforge/envs/sage`. Its separate
Python is 3.13.15; ordinary Python remains 3.12.13. The VM-local
`/home/agent/.local/bin/sage` wrapper supports the project's legacy `-python`
commands without activating Conda. Fresh SSH exact arithmetic, the full HKO
verifier in a temporary packet (4.40 seconds), and the pentagon 50-case prefix
(16.52 seconds) passed on 2026-09-05. The prefix is compatibility evidence,
not a full pentagon certificate rerun. Canonical outputs were preserved.
The setup occupies about 9.4 GB on the shared mount, with about 154 GB free;
VM-private free space remains about 2.2 GB. See [`INSTALL.md`](../INSTALL.md#sagemath)
for the tested commands, CA/network prerequisites and lifecycle constraints.

The host thesis build passed after regenerating bibliography intermediates
from another TeX version. When switching TeX environments, stale `build/`
contents can cause a Biber control-version mismatch; preserve any wanted PDF
before moving incompatible generated outputs aside and rebuilding.

A fresh sandbox-native thesis build also passed with latexmk 4.87, Biber 2.21
and pdfTeX 1.40.28 (TeX Live 2025/Debian), using a temporary output directory.
The nonempty log/PDF, PDF signature, overfull-box and undefined-reference checks
passed. Shared host build outputs were left untouched.

The Codex app-server runs inside the sandbox, and its agent actions and commands
execute there. The repository workspace is mounted into that environment,
while its installed packages, home directory, credentials, policies, and other
VM-private state can differ from the host.

The host runbook named above owns sandbox setup and recovery. Sandbox network
policy and reachability can differ from the host; do not assume that host
network access proves sandbox access.

Do not recreate the retired Paseo or T3 services, port publications, tunnels,
or client-specific SSH keys. The host runbook owns current sandbox access.

## Codex Cloud

Codex Cloud is a rare remote execution environment. The currently configured
environment uses:

```bash
scripts/bootstrap-cloud.sh
scripts/maintain-cloud.sh
```

The first is the setup script; the second is the maintenance script used after
a cached environment checks out the requested revision. Renaming either file
must be coordinated with the external Cloud environment configuration.

The setup currently expects Python 3.12 and Rust 1.94.0 and configures the
LaTeX, Cargo, and R2 support declared by the script. It consumes these setup
secrets:

```text
MSC_MATH_R2_ACCESS_KEY_ID
MSC_MATH_R2_SECRET_ACCESS_KEY
```

The script stores the required private rclone configuration without writing
credentials into Git. The script paths and secret names were confirmed in the
Cloud environment settings on 2026-08-29. That dated check does not establish
that the external settings remain unchanged: recheck them before diagnosing or
changing setup. Shell syntax was checked locally, but a fresh credentialed,
end-to-end setup was not rerun during this documentation pass.

See the [official Codex Cloud environment
documentation](https://learn.chatgpt.com/docs/environments/cloud-environment)
for setup, maintenance, caching, environment-variable, and secret lifecycle.

## Data and historical environments

The shared R2 bucket and its artifact registry remain current; see
[`artifacts.md`](artifacts.md). Each environment uses its standard XDG cache by
default, shared by all worktrees in that environment. Host and Docker Sandbox
may retain separate copies of the same R2 snapshot. Codex Cloud cache reuse is
opportunistic; only R2 provides durable cross-environment recovery.

Dated reports and benchmark records can accurately name a former devcontainer
or Compose environment as provenance for an old run. They do not define the
current development environment.
