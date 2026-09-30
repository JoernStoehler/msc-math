# Working environments and access paths

## Current intended setup

Jörn specified these project support targets on 2026-09-30 in Codex thread
`01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a`. This is user-confirmed working context,
not an end-to-end capability check or an implemented migration plan.

| Surface | Execution and intended use |
| --- | --- |
| Ubuntu workstation + Codex TUI | Persistent local execution for project work |
| claude.ai cloud microVM | Clone the repository into a separate cloud environment for agent work |
| ChatGPT Chat, Astra 6 Pro | Tough mathematical proof search, experiment brainstorming and reasoning; possibly writeup |

Jörn wants Pro for reasoning that would consume too much quota with weaker
models. This is his task-allocation preference, not a measured comparison of
model quality or cost. The project already has a
[human-mediated Pro handoff workflow](../.agents/skills/chatgpt-pro/SKILL.md).

Jörn reported that cold builds in the Claude microVM took under five minutes
when he last measured them. The command, revision, dependencies and machine
details were not specified here; do not extend this observation to Sage setup,
thesis builds, dataset downloads or full experiment runs.

### Jörn's terminal and browser access

All three terminal paths reach the Ubuntu workstation. They are clients of one
execution environment, not three separate agent environments.

| Device | Terminal access path |
| --- | --- |
| Ubuntu workstation | GNOME → kitty → Codex TUI |
| Chromebook | Tailscale → SSH with PTY → Codex TUI |
| Android phone | Tailscale → Termux → SSH → Codex TUI |

Companion Chrome sessions on these devices access project web surfaces over
Tailscale. Jörn wants those surfaces to show what is being worked on and what
is being neglected. The existing
[project dashboard](dashboard/README.md) is a starting surface; this statement
does not establish that it is currently reachable from each device or that it
provides complete activity/coverage reporting. A workstation-local file path
or localhost URL alone is not a remote-device review deliverable.

### Design implications and open choices

For telemetry, Jörn favours a generic OpenTelemetry storage solution on the
workstation, with Claude cloud and other external surfaces not exporting to it.
His deciding consideration is maintenance/setup cost; rare network outages are
not the main motivation. Later that day, in OTel thread
`01a0f1a8-2994-7ac2-abd9-d07386225de3`, he authorized workstation-only setup
through `~/.dotfiles`, trusting that session's OpenObserve recommendation.
The owner reports OpenObserve OSS v1.0.4 installed as an enabled user service,
with seven-day retention and 2 GiB/one-CPU caps. Native logs/traces, 90 metric
streams, PromQL and restart persistence passed without sending a model prompt.
The private UI is <https://joern-pc.tailc5e761.ts.net:8443/>; HTTPS/API checks
passed, while interactive browser/device checks were unavailable. These are
reported setup checks, not sustained-load or retention-expiry validation.
[The retained outcome](coordination/otel-design.md) links the setup/operations
owner at `~/.dotfiles/INSTALL.md`.

Jörn restarted the shared Codex daemon at `2026-09-30T10:32:33Z`. The owner
reports that the replacement process has all three signal authentication
variables and post-restart API queries returned 88 logs, 7,451 spans and two
metric series with two samples. Live workstation export is active; these are
timestamped observations, not completeness guarantees. Implementation is
complete: dotfiles commit `c2e8e54` owns the host setup and project commit
`20dd302d` retains the outcome. Maintenance belongs to `~/.dotfiles/INSTALL.md`;
credentials remain outside Git. Interactive browser login remains unverified.
External assignments and returned outcomes can still be represented in project
coordination without inventing their missing usage data.

The following are agent proposals or consequences of the topology, not selected
implementations:

- Local terminal sessions can share workstation storage, but separate
  worktrees still need explicit coordination and SSH reconnect/session
  persistence needs a documented mechanism.
- A Claude cloud clone has its own filesystem and runtime state. Whether it
  needs live messaging or detailed event records returned with its work remains
  undecided; neither is required by the preferred workstation-only telemetry
  direction. The workstation telemetry setup does not itself implement a
  semantic task/message journal.
- Pro research should receive focused context and return inspectable results;
  do not assume that Chat can clone the repository, run project commands or
  publish events to a service.
- Build setup and instructions should work from a fresh cloud clone without
  depending on workstation paths, installed plugin caches or host-only tools.
  The existing Codex Cloud bootstrap has not been validated for Claude.
- Quota/token/time observations may differ between surfaces. Comparable
  accounting and budget enforcement have not been established.

This discussion authorizes environment/workflow planning and recording, not an
autonomous thesis completion run. Recheck this section when Jörn changes his
devices, access routes or agent surfaces; verify live capabilities when a task
depends on them.

## Earlier Codex setup inventory

The records below describe the earlier Codex setup. Docker Sandbox and Codex
Cloud are retained setup/recovery information, not newly selected support
targets. Historical statements about availability, clients and running services
need a current check before use.

| Earlier environment | Recorded role |
| --- | --- |
| Host | direct execution environment and owner of host-only sandbox operations |
| Docker Sandbox (`sbx`) | intended project environment at the time; migration checks remained incomplete |
| Codex Cloud | rarely used remote environment for a selected repository revision |

An execution environment owns the processes, filesystem view, installed
software, network boundary, and runtime state for a thread. A Codex app-server
runs in that environment and serves the thread.

Codex TUI and the OpenAI extension for VS Code were the recorded local clients.
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

Codex skills are project-owned software under `.agents/skills`. The repository
contains its own byte-identical copy of the host-tested Herdr skill, so VM agents
can discover the CLI workflow without a global or cross-sandbox skills store.
Global memory remains a separate VM-local clone at `~/.agents/memories`.

The project config disables every known `/home/agent/.agents/skills` entry. The
retired shared VirtioFS mount was detached on 2026-09-14, but `sbx` will attach
it again after a VM restart until the sandbox is safely recreated with skill
sharing off. Do not edit it if it reappears. Always ask Jörn before recreating
this sandbox because recreation deletes VM-private state.

Make a reusable repository-owned skill correction as a focused commit. A host
coordinator can enumerate other repository consumers, fetch the source commit,
and apply it with `git cherry-pick -x` after checking each target's local skill
contract. A sandbox agent must not infer that it can see or update consumers in
other VMs.

Sage 10.9 is installed with Miniforge/conda-forge in the ignored shared path
`/workspaces/msc-math/.local-environments/miniforge/envs/sage`. Its separate
Python is 3.13.15; ordinary Python remains 3.12.13. Python files importing
`sage.all` run with the Sage environment's Python through
`conda run --name sage python`; the `sage` command itself remains the actual
Sage launcher for Sage files and version checks. Fresh SSH exact arithmetic,
the full HKO verifier in a temporary packet (4.40 seconds), and the pentagon
50-case prefix (16.52 seconds) passed on 2026-09-05. The prefix is compatibility
evidence, not a full pentagon certificate rerun. Canonical outputs were preserved.
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
