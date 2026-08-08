# Lifecycle interface; implementation and design rationale: container/

set shell := ["bash", "-euo", "pipefail", "-c"]

# List the development-environment commands.
default:
    @just --list

# Check host inputs and render the Compose configuration.
validate:
    @container/workspace.sh validate

# Build, smoke-test, and only then promote the local image tag.
build:
    @container/workspace.sh build

# Safely start an existing workspace, or create one without replacing it.
up:
    @container/workspace.sh up

# Explicitly replace the workspace container and its writable overlay.
replace:
    @container/workspace.sh replace

# Enter the workspace with an interactive login shell.
enter:
    @container/workspace.sh enter

# Install or update current vendor Codex in container-overlay home.
install-codex:
    @container/workspace.sh install-codex

# Perform one-time authentication and repository hook setup.
bootstrap:
    @container/workspace.sh bootstrap

# Start the authenticated Codex app-server in detached tmux.
app-server-up:
    @container/workspace.sh app-server-up

# Prove app-server process and listener readiness.
app-server-status:
    @container/workspace.sh app-server-status

# Show recent app-server output, including a retained failed pane.
app-server-logs:
    @container/workspace.sh app-server-logs

# Stop only the app-server process.
app-server-down:
    @container/workspace.sh app-server-down

# Start the workspace, current vendor Codex, and app-server.
agent-up:
    @container/workspace.sh agent-up

# Inspect the running core environment.
doctor:
    @container/workspace.sh doctor

# Inspect the app-server process, readiness, and loopback publication.
agent-doctor:
    @container/workspace.sh agent-doctor

# Report Compose, builder, network, image, and disk state.
status:
    @container/workspace.sh status

# Stop gracefully without deleting containers, networks, or host data.
stop:
    @container/workspace.sh stop

# Show replaceable BuildKit cache usage.
cache-usage:
    @container/workspace.sh cache-usage

# Interactively prune replaceable BuildKit cache.
cache-prune:
    @container/workspace.sh cache-prune
