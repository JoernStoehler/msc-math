#!/usr/bin/env bash
set -euo pipefail
source "$(dirname -- "${BASH_SOURCE[0]}")/common.sh"

validate
id="$(container_id)"
[[ -n "${id}" ]] && container_running "${id}" ||
  die "development container is not running"
assert_identity container "${id}"

compose exec -T "${SERVICE}" bash -lc \
  'set -euo pipefail
   test -w /workspaces/msc-math
   test -w "$CODEX_HOME"
   test -w "$HOME/.config/gh"
   test -w /data
   install -d /data/work /data/cache
   test -w /data/work
   test -w /data/cache
   test ! -S /var/run/docker.sock
   ! command -v docker >/dev/null
   probe="$HOME/.doctor-write-$$"
   trap '\''rm -f "$probe"; sudo -n rm -f /usr/local/share/.doctor-write-$$'\'' EXIT
   touch "$probe"
   sudo -n touch /usr/local/share/.doctor-write-$$
   test "$(npm prefix --global)" = "$HOME/.local"
   test "$(codex --version)" = "codex-cli '"$(env_value CODEX_VERSION)"'"
   rustc --version
   sage --version
   latexmk --version >/dev/null
   pre-commit --version
   rclone version >/dev/null
   hyperfine --version
   valgrind --version'

docker inspect "${id}" | jq -e \
  '.[0].HostConfig |
   .Memory == 10737418240 and
   .MemorySwap == 10737418240 and
   .ReadonlyRootfs == false and
   .PidsLimit == 16384' >/dev/null

docker inspect "${id}" | jq -e \
  --arg network "${NETWORK}" '.[0].NetworkSettings.Networks[$network] != null' >/dev/null
[[ "$(docker port "${id}" 4500/tcp)" == "127.0.0.1:${HOST_PORT}" ]]
curl -fsS --max-time 2 "http://127.0.0.1:${HOST_PORT}/readyz" >/dev/null
