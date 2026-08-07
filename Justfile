set dotenv-load := true
set shell := ["bash", "-euo", "pipefail", "-c"]

# List the development-environment commands.
default:
    @just --list

# Check host inputs and render the Compose configuration.
validate:
    test "$(uname -m)" = x86_64
    /lib64/ld-linux-x86-64.so.2 --help | grep -Fq 'x86-64-v3 (supported, searched)'
    test -f .env
    test "$(stat -c %a .env)" = 600
    git check-ignore -q .env
    test "$DEVELOPER_UID" = "$(id -u)"
    test "$DEVELOPER_GID" = "$(id -g)"
    test "$DEVELOPER_UID" -ge 1000
    test "$DEVELOPER_GID" -ge 1000
    command -v docker >/dev/null
    command -v git >/dev/null
    command -v jq >/dev/null
    docker compose version >/dev/null
    test -d "$CODEX_HOME_HOST" && test -w "$CODEX_HOME_HOST"
    test -d "$GH_HOME_HOST" && test -w "$GH_HOME_HOST"
    docker compose config -q

# Build with the dedicated constrained Buildx builder.
build: validate
    @if docker buildx inspect msc-math-builder >/dev/null 2>&1; then \
      options="$(docker buildx inspect msc-math-builder | sed -n 's/^Driver Options:[[:space:]]*//p')"; \
      grep -Fq 'memory="10g"' <<<"$options"; \
      grep -Fq 'memory-swap="10g"' <<<"$options"; \
    else \
      docker buildx create --name msc-math-builder --driver docker-container \
        --driver-opt memory=10g --driver-opt memory-swap=10g --use; \
    fi
    docker compose build --builder msc-math-builder --build-arg "DEVELOPER_UID=$DEVELOPER_UID" --build-arg "DEVELOPER_GID=$DEVELOPER_GID" --build-arg "WORKSPACE_REVISION=$(git rev-parse HEAD)" workspace
    docker run --rm --read-only \
      --tmpfs "/home/developer:rw,nosuid,nodev,exec,size=2g,uid=$DEVELOPER_UID,gid=$DEVELOPER_GID,mode=0700" \
      --tmpfs /tmp:rw,nosuid,nodev,exec,size=4g,mode=1777 \
      --entrypoint /bin/bash msc-math-workspace:local -lc 'rustc --version && sage --version && latexmk --version >/dev/null && pre-commit --version'

# Start the idle workspace, then install current vendor Codex.
up: validate
    docker compose up -d --no-build --pull never workspace
    just install-codex

# Install or update current vendor Codex in ephemeral home.
install-codex:
    docker compose exec -T workspace bash -lc 'flock "$HOME/.codex-install.lock" npm install --global --prefix "$HOME/.local" @openai/codex@latest && codex --version'

# Enter the workspace with an interactive login shell.
enter:
    docker compose exec workspace bash -l

# Perform one-time authentication and repository hook setup.
bootstrap:
    docker compose exec -T workspace bash -lc 'umask 077; test -s "$CODEX_HOME/app-server-token" || openssl rand -hex 32 >"$CODEX_HOME/app-server-token"; chmod 600 "$CODEX_HOME/app-server-token"'
    docker compose exec workspace bash -lc 'codex login status || codex login'
    docker compose exec workspace bash -lc 'gh auth status || gh auth login'
    docker compose exec -T workspace git lfs install --local
    docker compose exec -T workspace pre-commit install
    just doctor

# Start the authenticated Codex app-server in detached tmux.
app-server-up:
    docker compose exec -T workspace bash -lc 'test -s "$CODEX_HOME/app-server-token"; \
      if tmux has-session -t codex-app-server 2>/dev/null; then \
        curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null && exit 0; \
        tmux kill-session -t codex-app-server; \
      fi; \
      tmux new-session -d -s codex-app-server \
      "exec codex app-server --listen ws://0.0.0.0:4500 --ws-auth capability-token --ws-token-file $CODEX_HOME/app-server-token"; \
      for _ in {1..60}; do curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null && exit 0; sleep 0.5; done; \
      tmux capture-pane -pt codex-app-server -S -80 >&2 || true; exit 1'

# Show app-server process and readiness.
app-server-status:
    docker compose exec -T workspace bash -lc 'tmux has-session -t codex-app-server && curl --fail --silent --show-error http://127.0.0.1:4500/readyz'

# Show recent app-server output.
app-server-logs:
    docker compose exec -T workspace tmux capture-pane -pt codex-app-server -S -200

# Stop only the app-server process.
app-server-down:
    docker compose exec -T workspace bash -lc 'tmux kill-session -t codex-app-server 2>/dev/null || true'

# Replace the app-server bearer token.
rotate-app-server-token:
    docker compose exec -T workspace bash -lc 'tmux kill-session -t codex-app-server 2>/dev/null || true; \
      umask 077; token="$CODEX_HOME/.app-server-token.new"; openssl rand -hex 32 >"$token"; \
      mv "$token" "$CODEX_HOME/app-server-token"'
    @echo "Token rotated. Recreate clients that bind-mounted the old token file."

# Inspect the running environment.
doctor:
    docker compose exec -T workspace bash -lc 'test -w /workspaces/msc-math; \
      test -w "$CODEX_HOME"; test -w "$HOME/.config/gh"; \
      touch "$HOME/.doctor-write"; rm "$HOME/.doctor-write"; \
      ! touch /usr/.doctor-write 2>/dev/null; codex --version; rustc --version; \
      sage --version; latexmk --version | head -1; pre-commit --version'
    docker inspect "$(docker compose ps -q workspace)" | jq -e '.[0].HostConfig | .Memory == 10737418240 and .MemorySwap == 10737418240 and .ReadonlyRootfs == true and .PidsLimit == 16384'
    docker port "$(docker compose ps -q workspace)" 4500/tcp

# Report Compose, builder, network, and disk state.
status:
    docker compose ps
    @docker network inspect msc-math-dev 2>/dev/null | jq '.[0].Containers' || true
    @docker buildx inspect msc-math-builder 2>/dev/null || true
    @df -h .
    @docker system df

# Stop without deleting containers, networks, or host data.
stop:
    docker compose stop workspace

# Show replaceable BuildKit cache usage.
cache-usage:
    docker buildx du --builder msc-math-builder

# Interactively prune replaceable BuildKit cache.
cache-prune:
    docker buildx du --builder msc-math-builder
    docker buildx prune --builder msc-math-builder
