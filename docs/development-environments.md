# Development environments

Codex work on this repository currently runs in three execution environments:

| Environment | Role |
| --- | --- |
| Host | supported direct execution environment and owner of host-only sandbox operations |
| Docker Sandbox (`sbx`) | main local driver, normally entered from the host with `sbx run codex` |
| Codex Cloud | rarely used remote environment for work on a selected repository revision |

An execution environment owns the processes, filesystem view, installed
software, network boundary, and runtime state for a thread. A Codex app-server
runs in that environment and serves the thread.

Codex TUI, ChatGPT Desktop, and Paseo are clients through which Jörn reaches
threads. Each can be used with threads whose app-server runs in any of the
three environments. Thread execution occurs in the app-server environment; the
client therefore does not identify the execution environment.

## Tools and project contracts

The app-server advertises the function calls, tools, and MCP tools available to
the thread. That surface can differ with the thread's creation path and
execution environment. Agents use the advertised surface and its schemas; this
repository does not maintain a duplicate inventory or usage manual for it.
`.codex/config.toml` may configure operations, features, or integrations, but
the effective tool surface visible to a thread remains authoritative.

Project sources shared across environments own the task-level toolchain
contracts:

- `rust-toolchain.toml` and `Cargo.lock` own the Rust version and dependency
  graph;
- Python scripts with PEP 723 metadata run through `uv`;
- `AGENTS.md` owns baseline build and validation commands; and
- domain and producer READMEs own additional tools, commands, and output
  contracts.

Installed packages and executable availability can differ between
environments. Check the commands required by the task in the active environment
rather than inferring them from the client or from another thread.

## Host

The host is a supported execution environment, not merely a control plane for
another environment. It also owns Docker Sandbox creation, authentication,
policy, lifecycle, preservation, and recovery.

The cross-project host runbook is `/workspaces/DOCKER_SANDBOX.md`, outside this
repository. Read it before creating, repairing, or authenticating a sandbox.
Host-local sandbox state is not tracked project configuration.

## Docker Sandbox

Docker Sandbox is the main local driver. From the host checkout, the normal
entry command is:

```bash
sbx run codex
```

The Codex app-server runs inside the sandbox, and its agent actions and commands
execute there. The repository workspace is mounted into that environment,
while its installed packages, home directory, credentials, policies, and other
VM-private state can differ from the host.

The host runbook named above owns sandbox setup and recovery. Sandbox network
policy and reachability can differ from the host; do not assume that host
network access proves sandbox access.

Paseo installation, connectivity, port publication, lifecycle, and
verification are documented in `/workspaces/PASEO.md`, also outside this
repository.

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
credentials into Git. The current script paths and secrets were confirmed in
the Cloud environment settings on 2026-08-29; shell syntax was checked locally,
but a fresh end-to-end setup was not rerun during this documentation pass.

See the [official Codex Cloud environment
documentation](https://learn.chatgpt.com/docs/environments/cloud-environment)
for setup, maintenance, caching, environment-variable, and secret lifecycle.

## Data and historical environments

The shared R2 bucket and its artifact registry remain current; see
[`artifacts.md`](artifacts.md). The desired local data and cache layout across
host, Docker Sandbox, and Codex Cloud is not yet settled. Do not infer a shared
mount, cross-environment cache, or persistence guarantee from an
environment-local path.

Dated reports and benchmark records can accurately name a former devcontainer
or Compose environment as provenance for an old run. They do not define the
current development environment.
