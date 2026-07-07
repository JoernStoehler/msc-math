#!/usr/bin/env bash
set -euo pipefail

# Update /usr/local/bin/code-tunnel inside the existing devcontainer.
# This is host-side maintenance: it does not rebuild, recreate, start, or stop
# the container, so container-local state is preserved.

usage() {
  cat <<'EOF'
Usage: .devcontainer/host-update-vscode-tunnel.sh [--container CONTAINER] [--target PATH]

Download the latest stable VS Code CLI and copy it into the existing
devcontainer as /usr/local/bin/code-tunnel.

By default, the script finds the devcontainer by its Dev Containers
devcontainer.local_folder label. Use --container only if discovery is
ambiguous or the label is missing.

Options:
  --container CONTAINER  Docker container id or name to update
  --target PATH          Container path to update (default: /usr/local/bin/code-tunnel)
  -h, --help             Show this help

Requires:
  - docker
  - curl
  - tar
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
TARGET_PATH="/usr/local/bin/code-tunnel"
CONTAINER=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --container)
      if [[ $# -lt 2 ]]; then
        echo "[host-update-vscode-tunnel] --container requires a value" >&2
        exit 2
      fi
      CONTAINER="$2"
      shift 2
      ;;
    --target)
      if [[ $# -lt 2 ]]; then
        echo "[host-update-vscode-tunnel] --target requires a value" >&2
        exit 2
      fi
      TARGET_PATH="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[host-update-vscode-tunnel] unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

for command_name in docker curl tar; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "[host-update-vscode-tunnel] missing required command: $command_name" >&2
    exit 1
  fi
done

if [[ -z "$CONTAINER" ]]; then
  mapfile -t discovered_containers < <(docker ps -aq --filter "label=devcontainer.local_folder=${REPO_ROOT}")

  if [[ ${#discovered_containers[@]} -eq 0 ]]; then
    repo_name="$(basename "$REPO_ROOT")"
    mapfile -t discovered_containers < <(docker ps -aq --filter "name=vsc-${repo_name}-")
  fi

  case "${#discovered_containers[@]}" in
    0)
      echo "[host-update-vscode-tunnel] no existing devcontainer found for ${REPO_ROOT}" >&2
      echo "[host-update-vscode-tunnel] pass --container CONTAINER if Docker labels are missing" >&2
      exit 1
      ;;
    1)
      CONTAINER="${discovered_containers[0]}"
      ;;
    *)
      echo "[host-update-vscode-tunnel] multiple matching containers found for ${REPO_ROOT}:" >&2
      docker ps -a \
        --filter "label=devcontainer.local_folder=${REPO_ROOT}" \
        --format '  {{.ID}}  {{.Names}}  {{.Status}}' >&2
      echo "[host-update-vscode-tunnel] rerun with --container CONTAINER" >&2
      exit 1
      ;;
  esac
fi

if ! docker container inspect "$CONTAINER" >/dev/null 2>&1; then
  echo "[host-update-vscode-tunnel] Docker container not found: $CONTAINER" >&2
  exit 1
fi

container_status="$(docker inspect --format '{{.State.Status}}' "$CONTAINER")"
download_url="https://update.code.visualstudio.com/latest/cli-linux-x64/stable"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

echo "[host-update-vscode-tunnel] repository: ${REPO_ROOT}"
echo "[host-update-vscode-tunnel] container: ${CONTAINER} (${container_status})"
echo "[host-update-vscode-tunnel] downloading: ${download_url}"

curl -fsSL "$download_url" -o "$tmpdir/vscode-cli.tar.gz"
tar -xzf "$tmpdir/vscode-cli.tar.gz" -C "$tmpdir"

if [[ ! -f "$tmpdir/code" ]]; then
  echo "[host-update-vscode-tunnel] downloaded archive did not contain ./code" >&2
  exit 1
fi

chmod 0755 "$tmpdir/code"
downloaded_version="$("$tmpdir/code" --version)"
echo "[host-update-vscode-tunnel] downloaded: ${downloaded_version}"

docker cp "$tmpdir/code" "${CONTAINER}:${TARGET_PATH}"

docker cp "${CONTAINER}:${TARGET_PATH}" "$tmpdir/installed-code-tunnel"
chmod 0755 "$tmpdir/installed-code-tunnel"
installed_version="$("$tmpdir/installed-code-tunnel" --version)"

if [[ "$installed_version" != "$downloaded_version" ]]; then
  echo "[host-update-vscode-tunnel] verification mismatch" >&2
  echo "[host-update-vscode-tunnel] downloaded: ${downloaded_version}" >&2
  echo "[host-update-vscode-tunnel] installed:  ${installed_version}" >&2
  exit 1
fi

echo "[host-update-vscode-tunnel] installed at ${TARGET_PATH}: ${installed_version}"

if [[ "$container_status" == "running" ]]; then
  echo "[host-update-vscode-tunnel] container is running; new tunnel launches use the updated binary"
  echo "[host-update-vscode-tunnel] any already-running tunnel process must be restarted to use it"
else
  echo "[host-update-vscode-tunnel] container was not started or recreated"
fi
