#!/usr/bin/env bash
set -euo pipefail
source "$(dirname -- "${BASH_SOURCE[0]}")/common.sh"

start() {
  local id
  validate
  ensure_network
  id="$(container_id)"
  if [[ -n "${id}" ]]; then
    lock
    assert_identity container "${id}"
    if ! container_running "${id}"; then
      compose start "${SERVICE}" >/dev/null
    fi
  else
    if ! docker image inspect "${IMAGE}" >/dev/null 2>&1; then
      "${REPO_ROOT}/container/image.sh" build
    fi
    lock
    # Another concurrent start may have created it while the image was built.
    id="$(container_id)"
    if [[ -n "${id}" ]]; then
      assert_identity container "${id}"
      if ! container_running "${id}"; then
        compose start "${SERVICE}" >/dev/null
      fi
    else
    assert_identity image "${IMAGE}"
    compose up -d --no-build --pull never --no-recreate "${SERVICE}"
    id="$(container_id)"
    fi
  fi
  [[ -n "${id}" ]] || die "Compose did not create ${PROJECT}/${SERVICE}"
  assert_identity container "${id}"
  compose exec -T "${SERVICE}" bash -lc \
    'test "$(codex --version)" = "codex-cli '"$(env_value CODEX_VERSION)"'"'
}

stop() {
  local id
  validate
  lock
  id="$(container_id)"
  if [[ -n "${id}" ]] && container_running "${id}"; then
    "${REPO_ROOT}/container/app-server.sh" stop
    docker stop --time 30 "${id}" >/dev/null
  fi
}

recreate() {
  local answer
  validate
  lock
  docker image inspect "${IMAGE}" >/dev/null 2>&1 ||
    die "${IMAGE} is absent; run 'just image-build'"
  assert_identity image "${IMAGE}"
  if [[ "${CONFIRM_RECREATE:-}" != yes ]]; then
    read -r -p "Discard the container writable overlay and recreate it? [y/N] " answer
    [[ "${answer}" == y || "${answer}" == Y ]] || die "recreate cancelled"
  fi
  ensure_network
  compose up -d --no-build --pull never --force-recreate "${SERVICE}"
}

status() {
  local id desired_hash actual_hash image_id container_image
  validate
  compose ps --all
  id="$(container_id)"
  if docker image inspect "${IMAGE}" >/dev/null 2>&1; then
    image_id="$(docker image inspect --format '{{.Id}}' "${IMAGE}")"
    printf 'image=%s id=%s\n' "${IMAGE}" "${image_id}"
  else
    note "image ${IMAGE}: absent"
  fi
  if [[ -n "${id}" ]]; then
    actual_hash="$(docker inspect --format '{{index .Config.Labels "com.docker.compose.config-hash"}}' "${id}")"
    desired_hash="$(compose config --hash "${SERVICE}" | awk -v service="${SERVICE}" '$1 == service { print $2 }')"
    [[ -n "${desired_hash}" ]] || die "Compose did not report a configuration hash for ${SERVICE}"
    container_image="$(docker inspect --format '{{.Image}}' "${id}")"
    printf 'container=%s config=%s image=%s\n' "${id}" "${actual_hash}" "${container_image}"
    [[ "${actual_hash}" == "${desired_hash}" ]] || note "configuration drift: explicit container-recreate required"
    [[ -z "${image_id:-}" || "${container_image}" == "${image_id}" ]] ||
      note "image drift: explicit container-recreate required"
  fi
}

case "${1:-}" in
  start) start ;;
  stop) stop ;;
  recreate) recreate ;;
  status) status ;;
  *) die "usage: $0 {start|stop|recreate|status}" ;;
esac
