# Public host interface. Compose handles normal lifecycle; narrow helpers live under container/.

set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Normal: check host inputs without changing runtime state.
validate:
    @bash -c 'source container/common.sh; validate'

# Normal: start the container and authenticated app-server without rebuilding or recreating.
dev-start:
    @container/lifecycle.sh start
    @container/app-server.sh start
    @container/app-server.sh status

# Normal: stop processes and container while preserving durable state and the overlay.
dev-stop:
    @container/lifecycle.sh stop

# Normal: prove container, app-server, publication, and core environment health.
dev-status:
    @container/lifecycle.sh status
    @container/app-server.sh status
    @container/doctor.sh

# Normal: enter the running development container.
shell:
    @bash -c 'source container/common.sh; validate; compose exec workspace bash -l'

# Setup: authenticate this deployment's independent Codex and GitHub state.
auth-bootstrap:
    @bash -c 'source container/common.sh; validate; compose exec workspace bash -lc "codex login status || codex login"; compose exec workspace bash -lc "gh auth status || gh auth login"; compose exec -T workspace pre-commit install'

# Diagnostic: inspect the running core environment.
doctor:
    @container/doctor.sh

# Diagnostic: show recent app-server output, including a retained failed pane.
app-server-logs:
    @container/app-server.sh logs

# Advanced: start only the app-server in an already-running container.
app-server-start:
    @container/app-server.sh start

# Advanced: stop only the app-server.
app-server-stop:
    @container/app-server.sh stop

# Maintenance: build and smoke-test an image without replacing the container.
image-build:
    @container/image.sh build

# Maintenance: report container, image, builder, network, and disk state.
image-status:
    @container/lifecycle.sh status

# Destructive: replace the container and discard only its writable overlay.
container-recreate:
    @container/lifecycle.sh recreate

# Maintenance: show replaceable BuildKit cache usage.
cache-usage:
    @container/image.sh cache-usage

# Destructive: interactively prune replaceable BuildKit cache.
cache-prune:
    @container/image.sh cache-prune
