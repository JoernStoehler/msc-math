#!/usr/bin/env bash
# Project-local host helpers. A copied project edits the constants below.

readonly REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
readonly PROJECT="msc-math"
readonly SERVICE="workspace"
readonly IMAGE="msc-math-workspace:local"
readonly ENV_FILE="${MSC_MATH_ENV_FILE:-${REPO_ROOT}/.env}"
readonly NETWORK="joern-dev"
readonly NETWORK_LABEL="io.joern.dev.owner"
readonly NETWORK_LABEL_VALUE="joern-dev"
readonly HOST_PORT="4500"
readonly UID_LABEL="io.joern.msc-math.developer-uid"
readonly GID_LABEL="io.joern.msc-math.developer-gid"

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

host_only() {
  if [[ -f /.dockerenv ]]; then
    die "this controls the host container; run it on the Docker host in ${REPO_ROOT}"
  fi
}

compose() {
  env -u COMPOSE_FILE -u COMPOSE_PROJECT_NAME -u COMPOSE_PROFILES \
    -u COMPOSE_ENV_FILES -u DEVELOPER_UID -u DEVELOPER_GID \
    -u CODEX_HOME_HOST -u GH_HOME_HOST -u DATA_HOME_HOST COMPOSE_REMOVE_ORPHANS=0 \
    docker compose \
      --project-name "${PROJECT}" \
      --project-directory "${REPO_ROOT}" \
      --env-file "${ENV_FILE}" \
      --file "${REPO_ROOT}/compose.yaml" "$@"
}

env_value() {
  local key="$1" value
  value="$(sed -n "s/^${key}=//p" "${ENV_FILE}")"
  [[ -n "${value}" && "${value}" != *$'\n'* ]] ||
    die "${ENV_FILE} must define ${key} exactly once and nonempty"
  printf '%s' "${value}"
}

private_directory() {
  local label="$1" path="$2" expected_uid="$3" expected_gid="$4"
  local mode owner_uid owner_gid canonical
  [[ "${path}" = /* ]] || die "${label} must be absolute: ${path}"
  canonical="$(realpath -e -- "${path}")"
  [[ "${canonical}" == "${path}" ]] || die "${label} must be canonical: ${path}"
  [[ -d "${path}" && -w "${path}" ]] || die "${label} is not writable: ${path}"
  read -r mode owner_uid owner_gid < <(stat -c '%a %u %g' -- "${path}")
  [[ "${owner_uid}:${owner_gid}" == "${expected_uid}:${expected_gid}" ]] ||
    die "${label} must be owned by ${expected_uid}:${expected_gid}"
  (( (8#${mode} & 077) == 0 )) ||
    die "${label} must not be accessible by group/others (mode ${mode})"
}

persistent_directory() {
  local label="$1" path="$2" expected_uid="$3" expected_gid="$4"
  local owner_uid owner_gid canonical
  [[ "${path}" = /* ]] || die "${label} must be absolute: ${path}"
  canonical="$(realpath -e -- "${path}")"
  [[ "${canonical}" == "${path}" ]] || die "${label} must be canonical: ${path}"
  [[ -d "${path}" && -w "${path}" ]] || die "${label} is not writable: ${path}"
  read -r owner_uid owner_gid < <(stat -c '%u %g' -- "${path}")
  [[ "${owner_uid}:${owner_gid}" == "${expected_uid}:${expected_gid}" ]] ||
    die "${label} must be owned by ${expected_uid}:${expected_gid}"
}

require_local_docker() {
  local endpoint context
  need docker
  if [[ -n "${DOCKER_CONTEXT:-}" ]]; then
    context="${DOCKER_CONTEXT}"
    endpoint="$(docker context inspect "${context}" --format '{{(index .Endpoints "docker").Host}}')"
  elif [[ -n "${DOCKER_HOST:-}" ]]; then
    endpoint="${DOCKER_HOST}"
  else
    context="$(docker context show)"
    endpoint="$(docker context inspect "${context}" --format '{{(index .Endpoints "docker").Host}}')"
  fi
  [[ "${endpoint}" == unix:///* ]] ||
    die "expected a local Unix-socket Docker context, found ${endpoint}"
}

validate() {
  local developer_uid developer_gid
  host_only
  for command in awk curl docker flock git jq realpath sed stat; do need "${command}"; done
  require_local_docker
  [[ "$(uname -m)" == x86_64 ]] || die "the workspace image requires x86_64"
  /lib64/ld-linux-x86-64.so.2 --help | grep -Fq 'x86-64-v3 (supported, searched)' ||
    die "the Sage lock requires an x86-64-v3 host"
  [[ -d "${REPO_ROOT}/.git" ]] ||
    die "run lifecycle commands from the primary checkout, not a linked worktree"
  [[ -f "${ENV_FILE}" ]] ||
    die "copy .env.example to .env and fill it in"
  [[ "$(stat -c %a -- "${ENV_FILE}")" == 600 ]] ||
    die "${ENV_FILE} must have mode 0600"
  git -C "${REPO_ROOT}" check-ignore -q -- "$(realpath --relative-to="${REPO_ROOT}" "${ENV_FILE}")" ||
    die "${ENV_FILE} must remain ignored"

  developer_uid="$(env_value DEVELOPER_UID)"
  developer_gid="$(env_value DEVELOPER_GID)"
  [[ "${developer_uid}" == "$(id -u)" && "${developer_gid}" == "$(id -g)" ]] ||
    die "configured developer identity must match host $(id -u):$(id -g)"
  private_directory CODEX_HOME_HOST "$(env_value CODEX_HOME_HOST)" "${developer_uid}" "${developer_gid}"
  private_directory GH_HOME_HOST "$(env_value GH_HOME_HOST)" "${developer_uid}" "${developer_gid}"
  persistent_directory DATA_HOME_HOST "$(env_value DATA_HOME_HOST)" "${developer_uid}" "${developer_gid}"
  docker compose version >/dev/null
  docker info >/dev/null
  compose config -q
}

lock() {
  local directory="/tmp/${PROJECT}-compose-${UID}"
  install -d -m 0700 "${directory}"
  exec 9>"${directory}/lifecycle.lock"
  flock 9
}

ensure_network() {
  local driver label
  if ! docker network inspect "${NETWORK}" >/dev/null 2>&1; then
    docker network create --driver bridge \
      --label "${NETWORK_LABEL}=${NETWORK_LABEL_VALUE}" "${NETWORK}" >/dev/null ||
      docker network inspect "${NETWORK}" >/dev/null
  fi
  driver="$(docker network inspect "${NETWORK}" --format '{{.Driver}}')"
  label="$(docker network inspect "${NETWORK}" --format "{{index .Labels \"${NETWORK_LABEL}\"}}")"
  [[ "${driver}" == bridge && "${label}" == "${NETWORK_LABEL_VALUE}" ]] ||
    die "${NETWORK} exists but is not the expected labeled bridge"
}

container_id() {
  local output
  output="$(docker ps -aq \
    --filter "label=com.docker.compose.project=${PROJECT}" \
    --filter "label=com.docker.compose.service=${SERVICE}")"
  [[ "${output}" != *$'\n'* ]] ||
    die "multiple ${PROJECT}/${SERVICE} containers exist; inspect them manually"
  printf '%s' "${output}"
}

container_running() {
  [[ "$(docker inspect --format '{{.State.Running}}' "$1")" == true ]]
}

assert_identity() {
  local kind="$1" object="$2" uid gid expected_uid expected_gid
  expected_uid="$(env_value DEVELOPER_UID)"
  expected_gid="$(env_value DEVELOPER_GID)"
  if [[ "${kind}" == image ]]; then
    uid="$(docker image inspect --format "{{index .Config.Labels \"${UID_LABEL}\"}}" "${object}")"
    gid="$(docker image inspect --format "{{index .Config.Labels \"${GID_LABEL}\"}}" "${object}")"
  else
    uid="$(docker inspect --format "{{index .Config.Labels \"${UID_LABEL}\"}}" "${object}")"
    gid="$(docker inspect --format "{{index .Config.Labels \"${GID_LABEL}\"}}" "${object}")"
  fi
  [[ "${uid}:${gid}" == "${expected_uid}:${expected_gid}" ]] ||
    die "${kind} identity is ${uid:-unset}:${gid:-unset}, expected ${expected_uid}:${expected_gid}"
}

token_check() {
  local token="${REPO_ROOT}/.app-server-token"
  [[ -s "${token}" && "$(stat -c %a -- "${token}")" == 600 ]] ||
    die "${token} must exist, be nonempty, and have mode 0600"
  git -C "${REPO_ROOT}" check-ignore -q .app-server-token ||
    die ".app-server-token must remain ignored"
}
