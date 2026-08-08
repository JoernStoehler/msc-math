#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
readonly SCRIPT_DIR
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd -P)"
readonly REPO_ROOT
readonly COMPOSE_FILE="${REPO_ROOT}/compose.yaml"
readonly IMAGE="msc-math-workspace:local"
readonly BUILDER="msc-math-builder"
readonly PROJECT="msc-math"
readonly LIFECYCLE_LOCK_DIR="/tmp/msc-math-compose-${UID}"
readonly LIFECYCLE_LOCK="${LIFECYCLE_LOCK_DIR}/lifecycle.lock"

if [[ -n "${MSC_MATH_ENV_FILE:-}" ]]; then
  ENV_FILE="$(realpath -e -- "${MSC_MATH_ENV_FILE}")"
elif [[ -f "${REPO_ROOT}/.env" ]]; then
  readonly ENV_FILE="${REPO_ROOT}/.env"
elif [[ "${REPO_ROOT}" == */.worktrees/* ]] &&
  [[ -f "${REPO_ROOT%%/.worktrees/*}/.env" ]]; then
  readonly ENV_FILE="${REPO_ROOT%%/.worktrees/*}/.env"
else
  ENV_FILE="${REPO_ROOT}/.env"
fi
readonly ENV_FILE

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

note() {
  printf '%s\n' "$*" >&2
}

need() {
  command -v "$1" >/dev/null 2>&1 || die "required host command not found: $1"
}

env_value() {
  local key="$1"
  local -a values=()
  mapfile -t values < <(sed -n "s/^${key}=//p" "${ENV_FILE}")
  ((${#values[@]} == 1)) || die "${ENV_FILE} must define ${key} exactly once"
  [[ -n "${values[0]}" ]] || die "${key} must not be empty in ${ENV_FILE}"
  printf '%s' "${values[0]}"
}

compose() {
  env \
    -u COMPOSE_FILE \
    -u COMPOSE_PROJECT_NAME \
    -u COMPOSE_PROFILES \
    -u COMPOSE_ENV_FILES \
    -u DEVELOPER_UID \
    -u DEVELOPER_GID \
    -u CODEX_HOME_HOST \
    -u GH_HOME_HOST \
    COMPOSE_REMOVE_ORPHANS=0 \
    docker compose \
      --project-name "${PROJECT}" \
      --project-directory "${REPO_ROOT}" \
      --env-file "${ENV_FILE}" \
      --file "${COMPOSE_FILE}" \
      "$@"
}

require_local_docker() {
  local docker_context docker_host
  need docker
  # DOCKER_CONTEXT overrides DOCKER_HOST in the Docker CLI. Resolve the
  # effective endpoint without contacting the selected daemon.
  if [[ -n "${DOCKER_CONTEXT:-}" ]]; then
    docker_context="${DOCKER_CONTEXT}"
    docker_host="$(docker context inspect "${docker_context}" --format '{{(index .Endpoints "docker").Host}}')"
  elif [[ -n "${DOCKER_HOST:-}" ]]; then
    docker_host="${DOCKER_HOST}"
  else
    docker_context="$(docker context show)"
    docker_host="$(docker context inspect "${docker_context}" --format '{{(index .Endpoints "docker").Host}}')"
  fi
  [[ "${docker_host}" == unix:///* ]] || die \
    "the workspace requires a local Unix-socket Docker context, found ${docker_host}"
}

acquire_lifecycle_lock() {
  local mode owner_uid
  need flock
  need stat
  if ! mkdir -m 0700 -- "${LIFECYCLE_LOCK_DIR}" 2>/dev/null; then
    [[ -d "${LIFECYCLE_LOCK_DIR}" && ! -L "${LIFECYCLE_LOCK_DIR}" ]] || die \
      "lifecycle lock directory is not a private directory: ${LIFECYCLE_LOCK_DIR}"
  fi
  read -r mode owner_uid < <(stat -c '%a %u' -- "${LIFECYCLE_LOCK_DIR}")
  [[ "${mode}" == 700 && "${owner_uid}" == "${UID}" ]] || die \
    "lifecycle lock directory must be mode 0700 and owned by UID ${UID}"
  exec 9>"${LIFECYCLE_LOCK}"
  flock 9
}

require_primary_worktree() {
  [[ -d "${REPO_ROOT}/.git" ]] || die \
    "runtime lifecycle commands must run from the primary checkout, not a linked worktree"
}

validate_private_directory() {
  local label="$1"
  local path="$2"
  local expected_uid="$3"
  local expected_gid="$4"
  local mode owner_uid owner_gid canonical

  [[ "${path}" = /* ]] || die "${label} must be an absolute path: ${path}"
  canonical="$(realpath -e -- "${path}")"
  [[ "${canonical}" == "${path}" ]] || die \
    "${label} must be canonical (configured ${path}, canonical ${canonical})"
  [[ -d "${path}" && -w "${path}" ]] || die "${label} must be a writable directory: ${path}"
  read -r mode owner_uid owner_gid < <(stat -c '%a %u %g' -- "${path}")
  [[ "${owner_uid}" == "${expected_uid}" && "${owner_gid}" == "${expected_gid}" ]] || die \
    "${label} must be owned by ${expected_uid}:${expected_gid}, found ${owner_uid}:${owner_gid}"
  (( (8#${mode} & 077) == 0 )) || die \
    "${label} must not be accessible by group or other users (mode ${mode})"
}

validate() {
  local developer_uid developer_gid codex_home_host gh_home_host

  for command in date docker git jq flock realpath sed stat; do need "${command}"; done
  [[ "$(uname -m)" == x86_64 ]] || die "the workspace image requires x86_64"
  /lib64/ld-linux-x86-64.so.2 --help | grep -Fq 'x86-64-v3 (supported, searched)' || die \
    "the Sage lock requires x86-64-v3"
  [[ -f "${ENV_FILE}" ]] || die "copy .env.example to ${REPO_ROOT}/.env and fill it in"
  [[ "$(stat -c %a -- "${ENV_FILE}")" == 600 ]] || die "${ENV_FILE} must have mode 0600"
  if [[ "${ENV_FILE}" == "${REPO_ROOT}/.env" ]]; then
    git -C "${REPO_ROOT}" check-ignore -q .env || die ".env must remain ignored"
  fi

  developer_uid="$(env_value DEVELOPER_UID)"
  developer_gid="$(env_value DEVELOPER_GID)"
  codex_home_host="$(env_value CODEX_HOME_HOST)"
  gh_home_host="$(env_value GH_HOME_HOST)"
  [[ "${developer_uid}" =~ ^[0-9]+$ && "${developer_uid}" -ge 1000 ]] || die \
    "DEVELOPER_UID must be an integer at least 1000"
  [[ "${developer_gid}" =~ ^[0-9]+$ && "${developer_gid}" -ge 1000 ]] || die \
    "DEVELOPER_GID must be an integer at least 1000"
  [[ "${developer_uid}" == "$(id -u)" ]] || die "DEVELOPER_UID does not match the host user"
  [[ "${developer_gid}" == "$(id -g)" ]] || die "DEVELOPER_GID does not match the host group"
  validate_private_directory CODEX_HOME_HOST "${codex_home_host}" "${developer_uid}" "${developer_gid}"
  validate_private_directory GH_HOME_HOST "${gh_home_host}" "${developer_uid}" "${developer_gid}"

  docker compose version >/dev/null
  docker buildx version >/dev/null
  docker info >/dev/null
  compose config -q
}

image_label() {
  docker image inspect --format "{{ index .Config.Labels \"$2\" }}" "$1"
}

assert_identity_labels() {
  local object="$1"
  local kind="$2"
  local expected_uid expected_gid actual_uid actual_gid
  expected_uid="$(env_value DEVELOPER_UID)"
  expected_gid="$(env_value DEVELOPER_GID)"
  if [[ "${kind}" == image ]]; then
    actual_uid="$(image_label "${object}" io.joern.msc-math.developer-uid)"
    actual_gid="$(image_label "${object}" io.joern.msc-math.developer-gid)"
  else
    actual_uid="$(docker inspect --format '{{ index .Config.Labels "io.joern.msc-math.developer-uid" }}' "${object}")"
    actual_gid="$(docker inspect --format '{{ index .Config.Labels "io.joern.msc-math.developer-gid" }}' "${object}")"
  fi
  [[ "${actual_uid}" == "${expected_uid}" && "${actual_gid}" == "${expected_gid}" ]] || die \
    "${kind} ${object} has developer identity ${actual_uid:-unlabelled}:${actual_gid:-unlabelled}; expected ${expected_uid}:${expected_gid}. Run 'just build' and explicitly replace stale containers."
}

workspace_revision() {
  if [[ -n "${WORKSPACE_REVISION:-}" ]]; then
    printf '%s' "${WORKSPACE_REVISION}"
  elif git -C "${REPO_ROOT}" rev-parse --verify HEAD >/dev/null 2>&1; then
    git -C "${REPO_ROOT}" rev-parse HEAD
  else
    die "cannot read this linked worktree's revision on the host; set WORKSPACE_REVISION to its exact commit"
  fi
}

ensure_builder() {
  local inspection
  if docker buildx inspect "${BUILDER}" >/dev/null 2>&1; then
    inspection="$(docker buildx inspect "${BUILDER}")"
    grep -Eq '^Driver:[[:space:]]+docker-container$' <<<"${inspection}" || die \
        "${BUILDER} exists with the wrong driver; inspect it and repair explicitly"
    grep -Eq 'memory="?10g"?' <<<"${inspection}" || die \
      "${BUILDER} does not declare memory=10g; inspect it and repair explicitly"
    grep -Eq 'memory-swap="?10g"?' <<<"${inspection}" || die \
      "${BUILDER} does not declare memory-swap=10g; inspect it and repair explicitly"
  else
    docker buildx create \
      --name "${BUILDER}" \
      --driver docker-container \
      --driver-opt memory=10g \
      --driver-opt memory-swap=10g >/dev/null
  fi
}

build_image() {
  local revision candidate candidate_id developer_uid developer_gid build_id
  validate
  acquire_lifecycle_lock
  ensure_builder
  revision="$(workspace_revision)"
  developer_uid="$(env_value DEVELOPER_UID)"
  developer_gid="$(env_value DEVELOPER_GID)"
  build_id="$(date -u +%Y%m%d%H%M%S)-${BASHPID}"
  candidate="msc-math-workspace:candidate-${revision:0:12}-${build_id}"

  docker buildx build \
    --builder "${BUILDER}" \
    --platform linux/amd64 \
    --load \
    --tag "${candidate}" \
    --build-arg "DEVELOPER_UID=${developer_uid}" \
    --build-arg "DEVELOPER_GID=${developer_gid}" \
    --build-arg "WORKSPACE_REVISION=${revision}" \
    --file "${REPO_ROOT}/container/Dockerfile" \
    "${REPO_ROOT}"
  candidate_id="$(docker image inspect --format '{{.Id}}' "${candidate}")"
  assert_identity_labels "${candidate_id}" image
  # The single-quoted program is expanded by the container shell, not here.
  # shellcheck disable=SC2016
  docker run --rm --entrypoint /bin/bash "${candidate_id}" -lc \
    'set -euo pipefail
     sudo -n true
     test "$(command -v python3)" = /usr/bin/python3
     test "$(command -v gcc)" = /usr/bin/gcc
     test "$(command -v pkg-config)" = /usr/bin/pkg-config
     python3 -c "import sympy"
     sage --version
     sage -c "import sage.all"
     rustc --version
     latexmk --version >/dev/null
     pre-commit --version
     hyperfine --version
     valgrind --version'
  docker tag "${candidate_id}" "${IMAGE}"
  note "promoted tested ${candidate_id} to ${IMAGE}; the running container was not replaced"
}

workspace_container_id() {
  local destination="$1"
  local output
  local -a ids=()
  if ! output="$(
    docker ps --all --quiet \
      --filter "label=com.docker.compose.project=${PROJECT}" \
      --filter 'label=com.docker.compose.service=workspace'
  )"; then
    die "failed to enumerate ${PROJECT} workspace containers"
  fi
  if [[ -n "${output}" ]]; then
    mapfile -t ids <<<"${output}"
  fi
  ((${#ids[@]} <= 1)) || die \
    "multiple ${PROJECT} workspace containers found; inspect them before any lifecycle action"
  printf -v "${destination}" '%s' "${ids[0]:-}"
}

container_running() {
  local container_id="$1" state
  if ! state="$(docker inspect --format '{{.State.Running}}' "${container_id}")"; then
    die "failed to inspect workspace container ${container_id}"
  fi
  case "${state}" in
    true) return 0 ;;
    false) return 1 ;;
    *) die "workspace container ${container_id} reported invalid running state: ${state}" ;;
  esac
}

workspace_running() {
  local container_id
  workspace_container_id container_id
  [[ -n "${container_id}" ]] || return 1
  container_running "${container_id}"
}

install_codex() {
  local before after app_server_was_running=false
  before="$(compose exec -T workspace bash -lc 'codex --version 2>/dev/null || true')"
  if compose exec -T workspace tmux has-session -t codex-app-server 2>/dev/null; then
    app_server_was_running=true
  fi
  # The single-quoted program is expanded by the container shell, not here.
  # shellcheck disable=SC2016
  compose exec -T workspace bash -lc \
    'set -euo pipefail
     exec 9>"$HOME/.codex-install.lock"
     flock 9
     export NPM_CONFIG_PREFIX="$HOME/.local"
     npm install --global --prefix "$HOME/.local" @openai/codex@latest
     codex --version
     test "$(npm prefix --global)" = "$HOME/.local"'
  after="$(compose exec -T workspace codex --version)"
  if [[ "${before}" != "${after}" ]]; then
    note "Codex changed from '${before:-missing}' to '${after}'; revalidating integrations"
    if [[ "${app_server_was_running}" == true ]]; then
      app_server_down_impl
      app_server_up_impl
    fi
  fi
}

up_workspace() {
  local container_id
  require_primary_worktree
  validate
  acquire_lifecycle_lock
  workspace_container_id container_id
  if [[ -n "${container_id}" ]]; then
    assert_identity_labels "${container_id}" container
    compose start workspace
  else
    docker image inspect "${IMAGE}" >/dev/null 2>&1 || die "${IMAGE} is absent; run 'just build' first"
    assert_identity_labels "${IMAGE}" image
    compose up -d --no-build --pull never --no-recreate workspace
  fi
  install_codex
}

replace_workspace() {
  require_primary_worktree
  validate
  acquire_lifecycle_lock
  docker image inspect "${IMAGE}" >/dev/null 2>&1 || die "${IMAGE} is absent; run 'just build' first"
  assert_identity_labels "${IMAGE}" image
  if workspace_running; then
    app_server_down_impl
  fi
  note "replacing the workspace container; its writable overlay will be discarded"
  compose up -d --no-build --pull never --force-recreate workspace
  install_codex
  doctor_impl
}

token_file_check() {
  [[ -s "${REPO_ROOT}/.app-server-token" ]] || die \
    "${REPO_ROOT}/.app-server-token must exist and be nonempty for the app-server"
  [[ "$(stat -c %a -- "${REPO_ROOT}/.app-server-token")" == 600 ]] || die \
    "${REPO_ROOT}/.app-server-token must have mode 0600"
  git -C "${REPO_ROOT}" check-ignore -q .app-server-token || die \
    ".app-server-token must remain ignored"
}

app_server_up_impl() {
  token_file_check
  # The single-quoted program is expanded by the container shell, not here.
  # shellcheck disable=SC2016
  compose exec -T workspace bash -lc \
    'set -euo pipefail
     token=/workspaces/msc-math/.app-server-token
     test -s "$token"
     test "$(stat -c %a "$token")" = 600
     if tmux has-session -t codex-app-server 2>/dev/null; then
       if test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0 \
         && curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null; then
         exit 0
       fi
       tmux kill-session -t codex-app-server
     fi
     tmux new-session -d -s codex-app-server \
       "tmux set-option -p remain-on-exit on; exec codex app-server --listen ws://0.0.0.0:4500 --ws-auth capability-token --ws-token-file /workspaces/msc-math/.app-server-token"
     for _ in {1..60}; do
       test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0 || break
       if curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null; then
         exit 0
       fi
       sleep 0.5
     done
     tmux capture-pane -pt codex-app-server -S -80 >&2 || true
     exit 1'
}

app_server_down_impl() {
  local container_id
  workspace_container_id container_id
  if [[ -z "${container_id}" ]] || ! container_running "${container_id}"; then
    return
  fi
  docker exec "${container_id}" bash -lc \
    'set -euo pipefail
     if ! tmux has-session -t codex-app-server 2>/dev/null; then exit 0; fi
     tmux send-keys -t codex-app-server C-c
     for _ in {1..50}; do
       curl --fail --silent --max-time 0.2 http://127.0.0.1:4500/readyz >/dev/null 2>&1 || break
       sleep 0.1
     done
     tmux kill-session -t codex-app-server 2>/dev/null || true
     ! curl --fail --silent --max-time 0.2 http://127.0.0.1:4500/readyz >/dev/null 2>&1'
}

doctor_impl() {
  # The single-quoted program is expanded by the container shell, not here.
  # shellcheck disable=SC2016
  compose exec -T workspace bash -lc \
    'set -euo pipefail
     test -w /workspaces/msc-math
     test -w "$CODEX_HOME"
     test -w "$HOME/.config/gh"
     probe="$HOME/.doctor-write-$$"
     trap '\''rm -f "$probe"; sudo -n rm -f /usr/local/share/.doctor-write-$$'\'' EXIT
     touch "$probe"
     sudo -n touch /usr/local/share/.doctor-write-$$
     test "$(npm prefix --global)" = "$HOME/.local"
     codex --version
     rustc --version
     sage --version
     latexmk --version >/dev/null
     pre-commit --version
     hyperfine --version
     valgrind --version'
  local container_id
  workspace_container_id container_id
  [[ -n "${container_id}" ]] || die "workspace container not found"
  docker inspect "${container_id}" | jq -e \
    '.[0].HostConfig | .Memory == 10737418240 and .MemorySwap == 10737418240 and .ReadonlyRootfs == false and .PidsLimit == 16384' >/dev/null
}

status() {
  local container_id
  need docker
  need jq
  docker ps --all \
    --filter "label=com.docker.compose.project=${PROJECT}" \
    --filter 'label=com.docker.compose.service=workspace'
  workspace_container_id container_id
  if [[ -n "${container_id}" ]]; then
    docker inspect "${container_id}" --format \
      'container={{.Id}} image={{.Image}} configured_image={{.Config.Image}} revision={{index .Config.Labels "org.opencontainers.image.revision"}}'
  else
    note "workspace container: absent"
  fi
  if docker image inspect "${IMAGE}" >/dev/null 2>&1; then
    docker image inspect "${IMAGE}" --format \
      'local_image={{.Id}} revision={{index .Config.Labels "org.opencontainers.image.revision"}} uid={{index .Config.Labels "io.joern.msc-math.developer-uid"}} gid={{index .Config.Labels "io.joern.msc-math.developer-gid"}}'
  else
    note "local image: absent"
  fi
  if docker network inspect msc-math-dev >/dev/null 2>&1; then
    docker network inspect msc-math-dev | jq '.[0].Containers'
  else
    note "network msc-math-dev: absent"
  fi
  if docker buildx inspect "${BUILDER}"; then :; else note "builder ${BUILDER}: absent or unhealthy"; fi
  df -h "${REPO_ROOT}"
  docker system df
}

main() {
  local command="${1:-}" container_id
  case "${command}" in
    validate|build|up|replace|enter|install-codex|bootstrap|app-server-up|app-server-status|app-server-logs|app-server-down|agent-up|doctor|agent-doctor|status|stop|cache-usage|cache-prune)
      require_local_docker
      ;;
  esac
  case "${command}" in
    validate) validate ;;
    build) build_image ;;
    up) up_workspace ;;
    replace) replace_workspace ;;
    enter) require_primary_worktree; compose exec workspace bash -l ;;
    install-codex)
      require_primary_worktree
      validate
      acquire_lifecycle_lock
      workspace_running || die "workspace is not running"
      install_codex
      ;;
    bootstrap)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      acquire_lifecycle_lock
      compose exec workspace bash -lc 'codex login status || codex login'
      compose exec workspace bash -lc 'gh auth status || gh auth login'
      compose exec -T workspace git lfs install --local
      compose exec -T workspace pre-commit install
      doctor_impl
      ;;
    app-server-up)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      acquire_lifecycle_lock
      app_server_up_impl
      ;;
    app-server-status)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      token_file_check
      # The single-quoted program is expanded by the container shell, not here.
      # shellcheck disable=SC2016
      compose exec -T workspace bash -lc \
        'set -euo pipefail
         tmux has-session -t codex-app-server
         test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0
         curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null'
      ;;
    app-server-logs)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      compose exec -T workspace tmux capture-pane -pt codex-app-server -S -200
      ;;
    app-server-down)
      require_primary_worktree
      acquire_lifecycle_lock
      app_server_down_impl
      ;;
    agent-up)
      up_workspace
      app_server_up_impl
      ;;
    doctor)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      doctor_impl
      ;;
    agent-doctor)
      require_primary_worktree
      workspace_running || die "workspace is not running"
      doctor_impl
      token_file_check
      # The single-quoted program is expanded by the container shell, not here.
      # shellcheck disable=SC2016
      compose exec -T workspace bash -lc \
        'set -euo pipefail
         tmux has-session -t codex-app-server
         test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0
         curl --fail --silent --max-time 1 http://127.0.0.1:4500/readyz >/dev/null'
      need curl
      workspace_container_id container_id
      [[ "$(docker port "${container_id}" 4500/tcp)" == "127.0.0.1:4500" ]] || die \
        "app-server port is not published only on 127.0.0.1:4500"
      curl --fail --silent --max-time 2 http://127.0.0.1:4500/readyz >/dev/null
      ;;
    status) status ;;
    stop)
      require_primary_worktree
      need docker
      acquire_lifecycle_lock
      app_server_down_impl
      workspace_container_id container_id
      if [[ -n "${container_id}" ]]; then
        docker stop --time 30 "${container_id}" >/dev/null
      else
        note "workspace container: absent"
      fi
      ;;
    cache-usage) docker buildx du --builder "${BUILDER}" ;;
    cache-prune)
      acquire_lifecycle_lock
      docker buildx du --builder "${BUILDER}"
      docker buildx prune --builder "${BUILDER}"
      ;;
    *)
      die "usage: $0 {validate|build|up|replace|enter|install-codex|bootstrap|app-server-up|app-server-status|app-server-logs|app-server-down|agent-up|doctor|agent-doctor|status|stop|cache-usage|cache-prune}"
      ;;
  esac
}

main "$@"
