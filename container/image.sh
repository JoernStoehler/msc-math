#!/usr/bin/env bash
set -euo pipefail
source "$(dirname -- "${BASH_SOURCE[0]}")/common.sh"

image_build() {
  local revision candidate candidate_id
  validate
  lock
  revision="$(git -C "${REPO_ROOT}" rev-parse HEAD)"
  candidate="${IMAGE%:*}:candidate-${revision:0:12}-${BASHPID}"
  trap 'docker image rm "'"${candidate}"'" >/dev/null 2>&1 || true' EXIT

  docker build --platform linux/amd64 --tag "${candidate}" \
    --build-arg "DEVELOPER_UID=$(env_value DEVELOPER_UID)" \
    --build-arg "DEVELOPER_GID=$(env_value DEVELOPER_GID)" \
    --build-arg "CODEX_VERSION=$(env_value CODEX_VERSION)" \
    --build-arg "WORKSPACE_REVISION=${revision}" \
    --file "${REPO_ROOT}/container/Dockerfile" "${REPO_ROOT}"

  candidate_id="$(docker image inspect --format '{{.Id}}' "${candidate}")"
  assert_identity image "${candidate_id}"
  docker run --rm --entrypoint /bin/bash \
    --env "EXPECTED_CODEX_VERSION=$(env_value CODEX_VERSION)" "${candidate_id}" -lc \
    'set -euo pipefail
     test "$(id -u)" != 0
     sudo -n true
     test "$(command -v python3)" = /usr/bin/python3
     test "$(command -v gcc)" = /usr/bin/gcc
     test "$(command -v pkg-config)" = /usr/bin/pkg-config
     test "$(codex --version)" = "codex-cli ${EXPECTED_CODEX_VERSION}"
     python3 -c "import sympy"
     sage --version
     sage -c "import sage.all"
     rustc --version
     latexmk --version >/dev/null
     pre-commit --version
     hyperfine --version
     valgrind --version'
  docker tag "${candidate_id}" "${IMAGE}"
  note "promoted tested ${candidate_id} to ${IMAGE}; no container was replaced"
}

case "${1:-}" in
  build) image_build ;;
  cache-usage) docker system df ;;
  cache-prune) docker builder prune ;;
  *) die "usage: $0 {build|cache-usage|cache-prune}" ;;
esac
