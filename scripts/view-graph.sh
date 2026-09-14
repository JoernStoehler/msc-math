#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/view-graph.sh [FILE]
       scripts/view-graph.sh --list

FILE defaults to tasks.dot. A basename or codex/task-graph/FILE resolves in
the live graph directory; an existing path is used directly.
EOF
}

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
git_common=$(git -C "$repo_root" rev-parse --path-format=absolute --git-common-dir)
graph_dir="$git_common/codex/task-graph"

case "${1:-}" in
  -h|--help)
    usage
    exit 0
    ;;
  --list)
    find "$graph_dir" -maxdepth 1 -type f -name '*.dot' -printf '%f\n' | sort
    exit 0
    ;;
esac

if (( $# > 1 )); then
  usage >&2
  exit 2
fi

requested=${1:-tasks.dot}
if [[ -f "$requested" ]]; then
  source_file=$requested
elif [[ "$requested" == codex/task-graph/* ]]; then
  source_file="$graph_dir/${requested#codex/task-graph/}"
elif [[ "$requested" != */* ]]; then
  [[ "$requested" == *.dot ]] || requested="$requested.dot"
  source_file="$graph_dir/$requested"
else
  source_file=$requested
fi

if [[ ! -f "$source_file" ]]; then
  printf 'Graph not found: %s\n' "$source_file" >&2
  exit 1
fi
if ! command -v graph-easy >/dev/null; then
  printf 'graph-easy is required to render %s\n' "$source_file" >&2
  exit 127
fi

if [[ -t 0 && -t 1 ]] && command -v less >/dev/null; then
  rendered=$(mktemp)
  trap 'rm -f "$rendered"' EXIT
  graph-easy --from=dot --as=boxart "$source_file" >"$rendered"
  less -S "$rendered"
else
  graph-easy --from=dot --as=boxart "$source_file"
fi
